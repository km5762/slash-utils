use super::Config;
use big_num::types::U384;
use big_num::BigUint;
use elliptic_curve::Point;

pub const P384: Config<U384> = Config {
    p: BigUint::new([
        4294967295, 0, 0, 4294967295, 4294967294, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295,
    ]),
    a: BigUint::new([
        4294967292, 0, 0, 4294967295, 4294967294, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295,
    ]),
    b: BigUint::new([
        3555470063, 713410797, 2318324125, 3327539597, 1343457114, 51644559, 4269883666, 404593774,
        3824692505, 2559444331, 3795773412, 3006345127,
    ]),
    g: Point::new(
        BigUint::new([
            1920338615, 978607672, 3210029420, 1426256477, 2186553912, 1509376480, 2343017368,
            1847409506, 4079005044, 2394015518, 3196781879, 2861025826,
        ]),
        BigUint::new([
            2431258207, 2051218812, 494829981, 174109134, 3052452032, 3923390739, 681186428,
            4176747965, 2459098153, 1570674879, 2519084143, 907533898,
        ]),
    ),
    n: BigUint::new([
        3435473267, 3974895978, 1219536762, 1478102450, 4097256927, 3345173889, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
    ]),
    _private: (),
};

#[cfg(test)]
mod tests {
    use crate::{configs::tests::SignTest, test_sign};

    use super::*;

    #[test]
    fn sign_and_verify_p384() {
        let private_key = "6B9D3DAD2E1B8C1C05B19875B6659F4DE23C3B667BF297BA9AA47740787137D896D5724E4C70A825F872C9EA60D2EDF5";
        let public_key = (
            "EC3A4E415B4E19A4568618029F427FA5DA9A8BC4AE92E02E06AAE5286B300C64DEF8F0EA9055866064A254515480BC13",
            "8015D9B72D7D57244EA8EF9AC0C621896708A59367F9DFB9F54CA84B3F1C9DB1288B231C3AE0D4FE7344FD2533264720",
        );

        test_sign!(SignTest {
            config: P384,
            private_key,
            public_key,
            k: "4471EF7518BB2C7C20F62EAE1C387AD0C5E8E470995DB4ACF694466E6AB096630F29E5938D25106C3C340045A2DB01A7",
            hash: "8151325dcdbae9e0ff95f9f9658432dbedfdb209",
            signature: (
                "EC748D839243D6FBEF4FC5C4859A7DFFD7F3ABDDF72014540C16D73309834FA37B9BA002899F6FDA3A4A9386790D4EB2",
                "A3BCFA947BEEF4732BF247AC17F71676CB31A847B9FF0CBC9C9ED4C1A5B3FACF26F49CA031D4857570CCB5CA4424A443"
            )
        });
        test_sign!(SignTest {
            config: P384,
            private_key,
            public_key,
            k: "A4E4D2F0E729EB786B31FC20AD5D849E304450E0AE8E3E341134A5C1AFA03CAB8083EE4E3C45B06A5899EA56C51B5879",
            hash: "9003e374bc726550c2c289447fd0533160f875709386dfa377bfd41c",
            signature: (
                "42356E76B55A6D9B4631C865445DBE54E056D3B3431766D0509244793C3F9366450F76EE3DE43F5A125333A6BE060122",
                "9DA0C81787064021E78DF658F2FBB0B042BF304665DB721F077A4298B095E4834C082C03D83028EFBF93A3C23940CA8D",
            )
        });
        test_sign!(SignTest {
            config: P384,
            private_key,
            public_key,
            k: "180AE9F9AEC5438A44BC159A1FCB277C7BE54FA20E7CF404B490650A8ACC414E375572342863C899F9F2EDF9747A9B60",
            hash: "af2bdbe1aa9b6ec1e2ade1d694f41fc71a831d0268e9891562113d8a62add1bf",
            signature: (
                "21B13D1E013C7FA1392D03C5F99AF8B30C570C6F98D4EA8E354B63A21D3DAA33BDE1E888E63355D92FA2B3C36D8FB2CD",
                "F3AA443FB107745BF4BD77CB3891674632068A10CA67E3D45DB2266FA7D1FEEBEFDC63ECCD1AC42EC0CB8668A4FA0AB0"
            )
        });
        test_sign!(SignTest {
            config: P384,
            private_key,
            public_key,
            k: "94ED910D1A099DAD3254E9242AE85ABDE4BA15168EAF0CA87A555FD56D10FBCA2907E3E83BA95368623B8C4686915CF9",
            hash: "b298e408a891706a0e2023981d17eadca3f8e916beb0d54e18effd23d7bd171f413d240a9588b337983e1988532938b0",
            signature: (
                "94EDBB92A5ECB8AAD4736E56C691916B3F88140666CE9FA73D64C4EA95AD133C81A648152E44ACF96E36DD1E80FABE46",
                "36fdc149cbbbbfea713818ecc24bee7f23dc2070b15084f4d0f6812d78c0e27fc671d4aa3975e11eec1631ac37a6e48d"
            )
        });
        test_sign!(SignTest {
            config: P384,
            private_key,
            public_key,
            k: "92FC3C7183A883E24216D1141F1A8976C5B0DD797DFA597E3D7B32198BD35331A4E966532593A52980D0E3AAA5E10EC3",
            hash: "39a5e04aaff7455d9850c605364f514c11324ce64016960d23d5dc57d3ffd8f49a739468ab8049bf18eef820cdb1ad6c",
            signature: (
                "ED0959D5880AB2D869AE7F6C2915C6D60F96507F9CB3E047C0046861DA4A799CFE30F35CC900056D7C99CD7882433709",
                "512C8CCEEE3890A84058CE1E22DBC2198F42323CE8ACA9135329F03C068E5112DC7CC3EF3446DEFCEB01A45C2667FDD5"
            )
        });
    }
}