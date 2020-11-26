use crate::models::support::Support;
use rocket_contrib::json::Json;

/*extern crate lettre;
extern crate lettre_email;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};*/



#[post("/", format="json", data = "<support>")]
pub fn create(support: Json<Support>) -> Json<Support> {
    /*let email = Message::builder()
        .from("<estocy.project@gmail.com>".parse().unwrap())
        .to("<henrique.fquick@gmail.com>".parse().unwrap())
        .subject("Happy new year")
        .body("Be happy!")
        .unwrap();

    let creds = Credentials::new("estocy.project@gmail.com".to_string(), "estocy_project_puc".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }*/

//    env_logger::init();

    support
}