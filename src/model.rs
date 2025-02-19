//
// Check out `quicktype`.
//   On GitHub: https://github.com/quicktype/quicktype
//   In action: https://app.quicktype.io
//

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    main: Main,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthT {
    pub token: String,
    pub expires: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    temp: f64,
}
