use super::Config;
use big_num::types::U640;
use big_num::BigUint;
use elliptic_curve::Point;

pub const P521: Config<U640> = Config {
    p: BigUint::new([
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 511, 0, 0, 0,
    ]),
    a: BigUint::new([
        4294967292, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 511, 0, 0, 0,
    ]),
    b: BigUint::new([
        1800421120, 4014284756, 1026307313, 896786312, 1001504519, 374522045, 3967718267,
        1444493649, 2398161377, 3098839441, 2578650611, 2732225115, 3062186222, 2459574688,
        2384239135, 2503915873, 81, 0, 0, 0,
    ]),
    g: Point::new(
        BigUint::new([
            3269836134, 4185816625, 2238333595, 860402625, 2734663902, 4263362855, 4024916264,
            2706071159, 1800224186, 4163415904, 88061217, 2623832377, 597013570, 2654915430,
            67430861, 2240677559, 198, 0, 0, 0,
        ]),
        BigUint::new([
            2681300560, 2294191222, 2725429824, 893153414, 1068304225, 3310401793, 1593058880,
            2548986521, 658400812, 397393175, 1469793384, 2566210633, 746396633, 1552572340,
            2587607044, 959015544, 280, 0, 0, 0,
        ]),
    ),
    n: BigUint::new([
        2436391945, 3144660766, 2308720558, 1001769400, 4144604624, 2144076104, 3207566955,
        1367771011, 4294967290, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 511, 0, 0, 0,
    ]),
    _private: (),
};

#[cfg(test)]
mod tests {
    use crate::{configs::tests::SignTest, test_sign};

    use super::*;

    #[test]
    fn sign_and_verify_p521() {
        let private_key = "FAD06DAA62BA3B25D2FB40133DA757205DE67F5BB0018FEE8C86E1B68C7E75CAA896EB32F1F47C70855836A6D16FCC1466F6D8FBEC67DB89EC0C08B0E996B83538";
        let public_key = (
            "1894550D0785932E00EAA23B694F213F8C3121F86DC97A04E5A7167DB4E5BCD371123D46E45DB6B5D5370A7F20FB633155D38FFA16D2BD761DCAC474B9A2F5023A4",
            "493101C962CD4D2FDDF782285E64584139C2F91B47F87FF82354D6630F746A28A0DB25741B5B34A828008B22ACC23F924FAAFBD4D33F81EA66956DFEAA2BFDFCF5",
        );

        test_sign!(SignTest {
            config: P521,
            private_key,
            public_key,
            k: "89C071B419E1C2820962321787258469511958E80582E95D8378E0C2CCDB3CB42BEDE42F50E3FA3C71F5A76724281D31D9C89F0F91FC1BE4918DB1C03A5838D0F9",
            hash: "8151325dcdbae9e0ff95f9f9658432dbedfdb209",
            signature: (
                "343B6EC45728975EA5CBA6659BBB6062A5FF89EEA58BE3C80B619F322C87910FE092F7D45BB0F8EEE01ED3F20BABEC079D202AE677B243AB40B5431D497C55D75D",
                "E7B0E675A9B24413D448B8CC119D2BF7B2D2DF032741C096634D6D65D0DBE3D5694625FB9E8104D3B842C1B0E2D0B98BEA19341E8676AEF66AE4EBA3D5475D5D16"
            )
        });
        test_sign!(SignTest {
            config: P521,
            private_key,
            public_key,
            k: "121415EC2CD7726330A61F7F3FA5DE14BE9436019C4DB8CB4041F3B54CF31BE0493EE3F427FB906393D895A19C9523F3A1D54BB8702BD4AA9C99DAB2597B92113F3",
            hash: "9003e374bc726550c2c289447fd0533160f875709386dfa377bfd41c",
            signature: (
                "1776331CFCDF927D666E032E00CF776187BC9FDD8E69D0DABB4109FFE1B5E2A30715F4CC923A4A5E94D2503E9ACFED92857B7F31D7152E0F8C00C15FF3D87E2ED2E",
                "50CB5265417FE2320BBB5A122B8E1A32BD699089851128E360E620A30C7E17BA41A666AF126CE100E5799B153B60528D5300D08489CA9178FB610A2006C254B41F",
            )
        });
        test_sign!(SignTest {
            config: P521,
            private_key,
            public_key,
            k: "EDF38AFCAAECAB4383358B34D67C9F2216C8382AAEA44A3DAD5FDC9C32575761793FEF24EB0FC276DFC4F6E3EC476752F043CF01415387470BCBD8678ED2C7E1A0",
            hash: "af2bdbe1aa9b6ec1e2ade1d694f41fc71a831d0268e9891562113d8a62add1bf",
            signature: (
                "1511BB4D675114FE266FC4372B87682BAECC01D3CC62CF2303C92B3526012659D16876E25C7C1E57648F23B73564D67F61C6F14D527D54972810421E7D87589E1A7",
                "4A171143A83163D6DF460AAF61522695F207A58B95C0644D87E52AA1A347916E4F7A72930B1BC06DBE22CE3F58264AFD23704CBB63B29B931F7DE6C9D949A7ECFC"
            )
        });
        test_sign!(SignTest {
            config: P521,
            private_key,
            public_key,
            k: "1546A108BC23A15D6F21872F7DED661FA8431DDBD922D0DCDB77CC878C8553FFAD064C95A920A750AC9137E527390D2D92F153E66196966EA554D9ADFCB109C4211",
            hash: "b298e408a891706a0e2023981d17eadca3f8e916beb0d54e18effd23d7bd171f413d240a9588b337983e1988532938b0",
            signature: (
                "1EA842A0E17D2DE4F92C15315C63DDF72685C18195C2BB95E572B9C5136CA4B4B576AD712A52BE9730627D16054BA40CC0B8D3FF035B12AE75168397F5D50C67451",
                "742A2FFA3A1DED1A0A220A350542A92149AD99BFAF7A66DA8C12004B3AA562F3522433A68178433579B4C6CD47C9D05B3E5C6613DE9D93EDAF9AE088472F2623A0"
            )
        });
        test_sign!(SignTest {
            config: P521,
            private_key,
            public_key,
            k: "1DAE2EA071F8110DC26882D4D5EAE0621A3256FC8847FB9022E2B7D28E6F10198B1574FDD03A9053C08A1854A168AA5A57470EC97DD5CE090124EF52A2F7ECBFFD3",
            hash: "39a5e04aaff7455d9850c605364f514c11324ce64016960d23d5dc57d3ffd8f49a739468ab8049bf18eef820cdb1ad6c",
            signature: (
                "C328FAFCBD79DD77850370C46325D987CB525569FB63C5D3BC53950E6D4C5F174E25A1EE9017B5D450606ADD152B534931D7D4E8455CC91F9B15BF05EC36E377FA",
                "1FCF8B3C20108F5AD4465BAA04107E7164B227C31F355AC998D9644C9361423ECAC286823A38908EB4995C546CE6F07D538480CC30A43ECA5E9C094A16939F32771"
            )
        });
    }
}