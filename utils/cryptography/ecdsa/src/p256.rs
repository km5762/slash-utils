#[cfg(test)]
mod tests {
    use curves::P256;

    use crate::{test::SignTest, test_point_generation, test_sign};

    #[test]
    fn generate_point_p256() {
        test_point_generation!(
            P256,
            "2",
            "7CF27B188D034F7E8A52380304B51AC3C08969E277F21B35A60B48FC47669978",
            "7775510DB8ED040293D9AC69F7430DBBA7DADE63CE982299E04B79D227873D1"
        );
        test_point_generation!(
            P256,
            "3",
            "5ECBE4D1A6330A44C8F7EF951D4BF165E6C6B721EFADA985FB41661BC6E7FD6C",
            "8734640C4998FF7E374B06CE1A64A2ECD82AB036384FB83D9A79B127A27D5032"
        );
        test_point_generation!(
            P256,
            "4",
            "E2534A3532D08FBBA02DDE659EE62BD0031FE2DB785596EF509302446B030852",
            "E0F1575A4C633CC719DFEE5FDA862D764EFC96C3F30EE0055C42C23F184ED8C6"
        );
        test_point_generation!(
            P256,
            "5",
            "51590B7A515140D2D784C85608668FDFEF8C82FD1F5BE52421554A0DC3D033ED",
            "E0C17DA8904A727D8AE1BF36BF8A79260D012F00D4D80888D1D0BB44FDA16DA4"
        );
        test_point_generation!(
            P256,
            "6",
            "B01A172A76A4602C92D3242CB897DDE3024C740DEBB215B4C6B0AAE93C2291A9",
            "E85C10743237DAD56FEC0E2DFBA703791C00F7701C7E16BDFD7C48538FC77FE2"
        );
        test_point_generation!(
            P256,
            "7",
            "8E533B6FA0BF7B4625BB30667C01FB607EF9F8B8A80FEF5B300628703187B2A3",
            "73EB1DBDE03318366D069F83A6F5900053C73633CB041B21C55E1A86C1F400B4"
        );
        test_point_generation!(
            P256,
            "8",
            "62D9779DBEE9B0534042742D3AB54CADC1D238980FCE97DBB4DD9DC1DB6FB393",
            "AD5ACCBD91E9D8244FF15D771167CEE0A2ED51F6BBE76A78DA540A6A0F09957E"
        );
        test_point_generation!(
            P256,
            "9",
            "EA68D7B6FEDF0B71878938D51D71F8729E0ACB8C2C6DF8B3D79E8A4B90949EE0",
            "2A2744C972C9FCE787014A964A8EA0C84D714FEAA4DE823FE85A224A4DD048FA"
        );
        test_point_generation!(
            P256,
            "10",
            "CEF66D6B2A3A993E591214D1EA223FB545CA6C471C48306E4C36069404C5723F",
            "878662A229AAAE906E123CDD9D3B4C10590DED29FE751EEECA34BBAA44AF0773"
        );
        test_point_generation!(
            P256,
            "11",
            "3ED113B7883B4C590638379DB0C21CDA16742ED0255048BF433391D374BC21D1",
            "9099209ACCC4C8A224C843AFA4F4C68A090D04DA5E9889DAE2F8EEFCE82A3740"
        );
        test_point_generation!(
            P256,
            "112233445566778899",
            "339150844EC15234807FE862A86BE77977DBFB3AE3D96F4C22795513AEAAB82F",
            "B1C14DDFDC8EC1B2583F51E85A5EB3A155840F2034730E9B5ADA38B674336A21"
        );
        test_point_generation!(
            P256,
            "112233445566778899112233445566778899",
            "1B7E046A076CC25E6D7FA5003F6729F665CC3241B5ADAB12B498CD32F2803264",
            "BFEA79BE2B666B073DB69A2A241ADAB0738FE9D2DD28B5604EB8C8CF097C457B"
        );
        test_point_generation!(
            P256,
            "115792089210356248762697446949407573529996955224135760342422259061068512044355",
            "54E77A001C3862B97A76647F4336DF3CF126ACBE7A069C5E5709277324D2920B",
            "A660E43D60BCE8BBDEDE073FA5D183C8E8E15898CAF6FF7E45837D09F2F4C8A"
        );
    }

    #[test]
    fn sign_p256() {
        let private_key = "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721";
        let public_key = (
            "60FED4BA255A9D31C961EB74C6356D68C049B8923B61FA6CE669622E60F29FB6",
            "7903FE1008B8BC99A41AE9E95628BC64F2F1B20C2D7E9F5177A3C294D4462299",
        );

        test_sign!(SignTest {
            config: P256,
            private_key,
            public_key,
            k: "882905F1227FD620FBF2ABF21244F0BA83D0DC3A9103DBBEE43A1FB858109DB4",
            hash: "8151325dcdbae9e0ff95f9f9658432dbedfdb209",
            signature: (
                "61340C88C3AAEBEB4F6D667F672CA9759A6CCAA9FA8811313039EE4A35471D32",
                "6D7F147DAC089441BB2E2FE8F7A3FA264B9C475098FDCF6E00D7C996E1B8B7EB"
            )
        });
        test_sign!(SignTest {
            config: P256,
            private_key,
            public_key,
            k: "103F90EE9DC52E5E7FB5132B7033C63066D194321491862059967C715985D473",
            hash: "9003e374bc726550c2c289447fd0533160f875709386dfa377bfd41c",
            signature: (
                "53B2FFF5D1752B2C689DF257C04C40A587FABABB3F6FC2702F1343AF7CA9AA3F",
                "B9AFB64FDC03DC1A131C7D2386D11E349F070AA432A4ACC918BEA988BF75C74C",
            )
        });
        test_sign!(SignTest {
            config: P256,
            private_key,
            public_key,
            k: "A6E3C57DD01ABE90086538398355DD4C3B17AA873382B0F24D6129493D8AAD60",
            hash: "af2bdbe1aa9b6ec1e2ade1d694f41fc71a831d0268e9891562113d8a62add1bf",
            signature: (
                "EFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716",
                "F7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8"
            )
        });
        test_sign!(SignTest {
            config: P256,
            private_key,
            public_key,
            k: "09F634B188CEFD98E7EC88B1AA9852D734D0BC272F7D2A47DECC6EBEB375AAD4",
            hash: "9a9083505bc92276aec4be312696ef7bf3bf603f4bbd381196a029f340585312",
            signature: (
                "EAFEA039B20E9B42309FB1D89E213057CBF973DC0CFC8F129EDDDC800EF7719",
                "4861F0491E6998B9455193E34E7B0D284DDD7149A74B95B9261F13ABDE940954"
            )
        });
        test_sign!(SignTest {
            config: P256,
            private_key,
            public_key,
            k: "5FA81C63109BADB88C1F367B47DA606DA28CAD69AA22C4FE6AD7DF73A7173AA5",
            hash: "39a5e04aaff7455d9850c605364f514c11324ce64016960d23d5dc57d3ffd8f4",
            signature: (
                "8496A60B5E9B47C825488827E0495B0E3FA109EC4568FD3F8D1097678EB97F00",
                "2362AB1ADBE2B8ADF9CB9EDAB740EA6049C028114F2460F96554F61FAE3302FE"
            )
        });
    }
}