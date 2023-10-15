use serde::{Deserialize, Serialize};

/*
 * `Claims` is used for encoding and decoding JWT tokens.
 *
 * - `username`   [String]: The username of the claimant.
 * - `user_id`    [i32]: The identifier for the user making the claim.
 * - `exp`        [usize]: The expiry time of the token.
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub username: String,
    pub user_id: i32,
    pub exp: usize,
}
