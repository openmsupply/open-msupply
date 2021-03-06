CREATE TYPE report_type AS ENUM (
    'OM_SUPPLY'
);

CREATE TYPE context_type AS ENUM (
    'INBOUND_SHIPMENT',
    'OUTBOUND_SHIPMENT',
    'REQUISITION',
    'STOCKTAKE',
    'RESOURCE'
);


CREATE TABLE report (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    type report_type NOT NULL,
    template TEXT NOT NULL,
    context context_type NOT NULL,
    comment TEXT
)