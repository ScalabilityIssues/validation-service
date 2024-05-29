use ed25519_dalek::{Signer, SigningKey};
use prost::Message;
use tonic::{Request, Response, Status};

use crate::proto::validationsvc::{
    validation_server::Validation, GetVerificationKeyResponse, SignTicketRequest,
    SignTicketResponse, SignedTicket,
};
use crate::qr;

pub struct ValidationApp {
    signing_key: SigningKey,
}

#[tonic::async_trait]
impl Validation for ValidationApp {
    async fn sign_ticket(
        &self,
        request: Request<SignTicketRequest>,
    ) -> Result<Response<SignTicketResponse>, Status> {
        let SignTicketRequest {
            ticket: Some(ticket),
        } = request.into_inner()
        else {
            return Err(Status::invalid_argument("Ticket is required"));
        };
        let ticket = ticket.encode_to_vec();

        let signature = self.signing_key.sign(&ticket).to_vec();

        let signed_ticket = SignedTicket { ticket, signature };

        let qr = qr::make_qr_code(signed_ticket)?;

        Ok(Response::new(SignTicketResponse { qr }))
    }

    async fn get_verification_keys(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetVerificationKeyResponse>, Status> {
        Ok(Response::new(GetVerificationKeyResponse {
            verification_keys: vec![self.signing_key.verifying_key().to_bytes().to_vec()],
        }))
    }
}

impl ValidationApp {
    pub fn new(signing_key: SigningKey) -> Self {
        Self { signing_key }
    }
}
