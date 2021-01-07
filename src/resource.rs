use p256::ecdsa::VerifyingKey;


struct Resource{
    name: String,
    default_tracker: String,
    pub_key: VerifyingKey,
}