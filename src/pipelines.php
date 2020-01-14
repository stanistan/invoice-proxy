<?php declare(strict_types=1);

namespace pipelines;

use FetchCtx;
use transforms as t;

function invoice(FetchCtx $fetch) {
    return pipeline(
        $fetch->invoice(),
        t\mapAndPickKeys(
            [ 'ID' ],
            [ 'Date' ],
            [ 'Due Date' ],
            [ 'Invoice Number' ],
            [ 'Total Amount', t\money() ],
            [ 'From', t\first(), from($fetch) ],
            [ 'Client', t\first(), client($fetch) ],
            [ 'Invoice Item', t\map(invoiceItem($fetch)) ],
        )
    );
}

function from(FetchCtx $fetch) {
    return pipeline(
        $fetch->me(),
        t\mapAndPickKeys(
            [ 'Name' ],
            [ 'Email' ],
            [ 'Address', fn($address) => explode("\n", $address) ]
        )
    );
}

function invoiceItem(FetchCtx $fetch) {
    return pipeline(
        $fetch->invoiceItem(),
        t\mapAndPickKeys(
            [ 'Date' ],
            [ 'Description' ],
            [ 'Quantity' ],
            [ 'Amount', t\money() ],
            [ 'Invoice Rate', t\first(), $fetch->invoiceRate(), t\pickKeys('Name', 'Notes') ],
        ),
    );
}

function client(FetchCtx $fetch) {
    return pipeline(
        $fetch->client(),
        t\pickKeys('ContactName', 'Company', 'Website', 'ContactEmail')
    );
}
