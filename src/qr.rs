use std::io::Cursor;

use base64::Engine;
use image::Luma;
use prost::Message;
use qrcode::QrCode;
use tonic::Status;

use crate::proto::validationsvc::SignedTicket;

pub fn make_qr_code(signed_ticket: SignedTicket) -> Result<Vec<u8>, QrError> {
    let data = signed_ticket.encode_to_vec();
    let data = base64::engine::general_purpose::STANDARD.encode(data);

    let qr = QrCode::new(data)?;

    let qr = qr.render::<Luma<u8>>().module_dimensions(1, 1).build();

    let mut v = Vec::new();
    qr.write_to(&mut Cursor::new(&mut v), image::ImageFormat::Png)?;

    Ok(v)
}

#[derive(Debug, thiserror::Error)]
pub enum QrError {
    #[error("Failed to create QR code")]
    QrCreationError(#[from] qrcode::types::QrError),
    #[error("Failed to write QR code")]
    ImageWriteError(#[from] image::ImageError),
}

impl From<QrError> for Status {
    fn from(e: QrError) -> Self {
        tracing::error!("Error generating QR: {}", e);
        Status::internal("Error creating QR code")
    }
}
