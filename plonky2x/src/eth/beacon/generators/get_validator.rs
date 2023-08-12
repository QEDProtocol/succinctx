use core::marker::PhantomData;

use itertools::Itertools;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::generator::{GeneratedValues, SimpleGenerator};
use plonky2::iop::target::Target;
use plonky2::iop::witness::{PartitionWitness, Witness};
use plonky2::plonk::circuit_data::CommonCircuitData;
use plonky2::util::serialization::{Buffer, IoResult};

use crate::eth::beacon::vars::BeaconValidatorWitnessWrite;
use crate::eth::beacon::BeaconValidatorVariable;
use crate::ethutils::beacon::BeaconClient;
use crate::vars::{Bytes32Variable, WitnessMethods};

#[derive(Debug)]
struct GetBeaconValidatorGenerator<F: RichField + Extendable<D>, const D: usize> {
    pub block_root: Bytes32Variable,
    pub validator_idx: u64,
    pub validator: BeaconValidatorVariable,
    pub client: BeaconClient,
    pub _phantom: PhantomData<F>,
}

impl<F: RichField + Extendable<D>, const D: usize> GetBeaconValidatorGenerator<F, D> {
    pub fn new(
        block_root: Bytes32Variable,
        validator_idx: u64,
        validator: BeaconValidatorVariable,
        client: BeaconClient,
    ) -> Self {
        Self {
            block_root,
            validator_idx,
            validator,
            client,
            _phantom: Default::default(),
        }
    }
}

impl<F: RichField + Extendable<D>, const D: usize> SimpleGenerator<F, D>
    for GetBeaconValidatorGenerator<F, D>
{
    fn id(&self) -> String {
        "GetBeaconValidatorGenerator".to_string()
    }

    fn dependencies(&self) -> Vec<Target> {
        self.block_root.0.into_iter().map(|x| x.0 .0).collect_vec()
    }

    fn run_once(&self, witness: &PartitionWitness<F>, out_buffer: &mut GeneratedValues<F>) {
        let block_root = witness.get_hex_string(self.block_root.into());
        println!("{}", block_root);
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt
            .block_on(self.client.get_validator(block_root, self.validator_idx))
            .unwrap();
        let value = result.validator;
        out_buffer.set_validator(self.validator, value);
    }

    #[allow(unused_variables)]
    fn serialize(&self, dst: &mut Vec<u8>, common_data: &CommonCircuitData<F, D>) -> IoResult<()> {
        todo!()
    }

    #[allow(unused_variables)]
    fn deserialize(src: &mut Buffer, common_data: &CommonCircuitData<F, D>) -> IoResult<Self> {
        todo!()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::env;

    use ethers::types::H256;
    use plonky2::iop::witness::PartialWitness;

    use super::GetBeaconValidatorGenerator;
    use crate::builder::BuilderAPI;
    use crate::eth::beacon::BeaconAPI;
    use crate::ethutils::beacon::BeaconClient;
    use crate::vars::WitnessWriteMethods;

    #[test]
    fn test_simple_circuit() {
        dotenv::dotenv().ok();

        let mut api = BuilderAPI::new();
        let block_root = api.init_bytes32();
        let mut beacon_api = BeaconAPI::new(&mut api, "".into());
        let validator = beacon_api.init_validator();
        let generator = GetBeaconValidatorGenerator::new(
            block_root,
            0,
            validator,
            BeaconClient::new(env::var("CONSENSUS_RPC_URL").unwrap().into()),
        );
        api.api.add_simple_generator(generator);

        let mut pw = PartialWitness::new();
        let block_root_raw = "0x6de59dc86b36b81bdae8cfdf9c9283e06fc78234a62cac274f2bef1fd1cfd209"
            .parse::<H256>()
            .unwrap();
        let block_root_value = block_root_raw.as_fixed_bytes();
        pw.set_from_bytes_be(block_root.into(), *block_root_value);

        let data = api.build();
        let proof = data.prove(pw).unwrap();
        data.verify(proof).unwrap();
    }
}