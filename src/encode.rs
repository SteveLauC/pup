use base64::encode;




mod test{
    use base64::encode;

    #[test]
    fn encode_test() {
        let str: &str = "my new file contents";
        let e_str = encode(str.as_bytes());
        
        assert_eq!(e_str.as_str(), "bXkgbmV3IGZpbGUgY29udGVudHM=");
    }
}