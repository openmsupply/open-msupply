use crate::{
    invoice::{
        check_invoice_exists, check_invoice_is_editable, check_invoice_type, check_store,
        validate::InvoiceIsNotEditable, InvoiceDoesNotExist, NotThisStoreInvoice,
        WrongInvoiceRowType,
    },
    invoice_line::{
        validate::{check_line_exists, LineDoesNotExist, NotInvoiceLine},
        DeleteInboundShipmentLine,
    },
};
use repository::{InvoiceLineRow, InvoiceRowType, StorageConnection};

use super::DeleteInboundShipmentServiceLineError;

pub fn validate(
    input: &DeleteInboundShipmentLine,
    store_id: &str,
    connection: &StorageConnection,
) -> Result<InvoiceLineRow, DeleteInboundShipmentServiceLineError> {
    let line = check_line_exists(&input.id, connection)?;
    let invoice = check_invoice_exists(&line.invoice_id, connection)?;

    check_store(&invoice, store_id)?;
    check_invoice_type(&invoice, InvoiceRowType::InboundShipment)?;
    check_invoice_is_editable(&invoice)?;

    Ok(line)
}

impl From<LineDoesNotExist> for DeleteInboundShipmentServiceLineError {
    fn from(_: LineDoesNotExist) -> Self {
        DeleteInboundShipmentServiceLineError::LineDoesNotExist
    }
}

impl From<WrongInvoiceRowType> for DeleteInboundShipmentServiceLineError {
    fn from(_: WrongInvoiceRowType) -> Self {
        DeleteInboundShipmentServiceLineError::NotAnInboundShipment
    }
}

impl From<InvoiceIsNotEditable> for DeleteInboundShipmentServiceLineError {
    fn from(_: InvoiceIsNotEditable) -> Self {
        DeleteInboundShipmentServiceLineError::CannotEditInvoice
    }
}

impl From<NotInvoiceLine> for DeleteInboundShipmentServiceLineError {
    fn from(error: NotInvoiceLine) -> Self {
        DeleteInboundShipmentServiceLineError::NotThisInvoiceLine(error.0)
    }
}

impl From<InvoiceDoesNotExist> for DeleteInboundShipmentServiceLineError {
    fn from(_: InvoiceDoesNotExist) -> Self {
        DeleteInboundShipmentServiceLineError::InvoiceDoesNotExist
    }
}

impl From<NotThisStoreInvoice> for DeleteInboundShipmentServiceLineError {
    fn from(_: NotThisStoreInvoice) -> Self {
        DeleteInboundShipmentServiceLineError::NotThisStoreInvoice
    }
}
