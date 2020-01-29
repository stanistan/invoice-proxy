<?php declare(strict_types=1);

namespace pipelines;

use FetchCtx;
use transforms as t;

function invoiceById(FetchCtx $fetch) {
    return pipeline($fetch->invoiceById(), invoiceFields($fetch));
}

function invoiceFields(FetchCtx $fetch) {
    return t\mapAndPickKeys(
        [ 'ID' ],
        [ 'Date' ],
        [ 'Due Date' ],
        [ 'Invoice Number' ],
        [ 'Total Amount', t\money() ],
        [ 'From', t\first(), from($fetch) ],
        [ 'Client', t\first(), client($fetch) ],
        [ 'Invoice Item', t\map(invoiceItem($fetch)) ],
    );
}

function invoice(FetchCtx $fetch) {
    return pipeline($fetch->invoice(), invoiceFields($fetch));
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
            [ 'Invoice Rate', t\first(), $fetch->invoiceRate(), t\mapAndPickKeys(
                [ 'Name' ],
                [ 'Notes' ],
                [ 'Rate' ],
                [ 'Unit', t\first(), $fetch->unit(), t\enter('Name') ]
            ) ],
        ),
    );
}

function client(FetchCtx $fetch) {
    return pipeline(
        $fetch->client(),
        t\pickKeys('ContactName', 'Company', 'Website', 'ContactEmail')
    );
}
