use common::challenge::models_recover_secret::{RecoverSecretInput, RecoverSecretOutput};

pub fn secret_challenge_resolver(input: RecoverSecretInput) -> RecoverSecretOutput {
    return RecoverSecretOutput {
        secret_sentence: String::from(""),
    };
}
