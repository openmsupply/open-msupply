use crate::{
    invoice::{
        check_invoice_exists, check_invoice_is_editable, check_invoice_type, check_store,
        validate::InvoiceIsNotEditable, InvoiceDoesNotExist, NotThisStoreInvoice,
        WrongInvoiceRowType,
    },
    invoice_line::{
        inbound_shipment_line::check_batch,
        validate::{check_line_exists, LineDoesNotExist, NotInvoiceLine},
        BatchIsReserved,
    },
};
use repository::{InvoiceLineRow, InvoiceRow, InvoiceRowType, StorageConnection};

use super::{DeleteInboundShipmentLine, DeleteInboundShipmentLineError};

pub fn validate(
    input: &DeleteInboundShipmentLine,
    store_id: &str,
    connection: &StorageConnection,
) -> Result<(InvoiceRow, InvoiceLineRow), DeleteInboundShipmentLineError> {
    let line = check_line_exists(&input.id, connection)?;

    let invoice = check_invoice_exists(&line.invoice_id, connection)?;
    check_store(&invoice, store_id)?;
    check_invoice_type(&invoice, InvoiceRowType::InboundShipment)?;
    check_invoice_is_editable(&invoice)?;
    check_batch(&line, connection)?;

    Ok((invoice, line))
}

impl From<LineDoesNotExist> for DeleteInboundShipmentLineError {
    fn from(_: LineDoesNotExist) -> Self {
        DeleteInboundShipmentLineError::LineDoesNotExist
    }
}

impl From<WrongInvoiceRowType> for DeleteInboundShipmentLineError {
    fn from(_: WrongInvoiceRowType) -> Self {
        DeleteInboundShipmentLineError::NotAnInboundShipment
    }
}

impl From<InvoiceIsNotEditable> for DeleteInboundShipmentLineError {
    fn from(_: InvoiceIsNotEditable) -> Self {
        DeleteInboundShipmentLineError::CannotEditFinalised
    }
}

impl From<NotInvoiceLine> for DeleteInboundShipmentLineError {
    fn from(error: NotInvoiceLine) -> Self {
        DeleteInboundShipmentLineError::NotThisInvoiceLine(error.0)
    }
}

impl From<BatchIsReserved> for DeleteInboundShipmentLineError {
    fn from(_: BatchIsReserved) -> Self {
        DeleteInboundShipmentLineError::BatchIsReserved
    }
}

impl From<InvoiceDoesNotExist> for DeleteInboundShipmentLineError {
    fn from(_: InvoiceDoesNotExist) -> Self {
        DeleteInboundShipmentLineError::InvoiceDoesNotExist
    }
}

impl From<NotThisStoreInvoice> for DeleteInboundShipmentLineError {
    fn from(_: NotThisStoreInvoice) -> Self {
        DeleteInboundShipmentLineError::NotThisStoreInvoice
    }
}
