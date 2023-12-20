// {"access_token":"BQAqVCJMRm4g7SZG8lIn64-tvfwdSQoEL6iYNt24O9UhO6BkidiEuRmHVsO2dDiif_aVWOPm93GTL7wfoJ5y7ffClJN7h5lAR-tjbNkElbcd4bAYog8","token_type":"Bearer","expires_in":3600}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken {
    access_token: String, 
    token_type: String,
    expires_in: i64,
}

impl std::fmt::Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {} {}", self.access_token, self.token_type, self.expires_in)
    }
}



