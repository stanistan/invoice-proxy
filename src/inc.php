<?php declare(strict_types=1);

require_once __DIR__ . '/transforms.php';
require_once __DIR__ . '/Response.php';
require_once __DIR__ . '/Pipeline.php';
require_once __DIR__ . '/FetchCtx.php';

use transforms as t;

class Pipelines {

    private FetchCtx $fetch;

    public function __construct(FetchCtx $req_ctx) {
        $this->fetch = $req_ctx;
    }

    public function invoice() {
        return pipeline(
            $this->fetch->invoice(),
            t\fields(),
            t\mapAndPickKeys(
                [ 'ID' ],
                [ 'Date' ],
                [ 'Due Date' ],
                [ 'Invoice Number' ],
                [ 'Total Amount', t\money() ],
                [ 'From', t\first(), $this->from() ],
                [ 'Client', t\first(), $this->client() ],
                [ 'Invoice Item', t\map($this->invoiceItem()) ],
            )
        );
    }

    public function invoiceItem() {
        return pipeline(
            $this->fetch->invoiceItem(),
            t\fields(),
            t\mapAndPickKeys(
                [ 'Date' ],
                [ 'Description' ],
                [ 'Quantity' ],
                [ 'Amount', t\money() ],
                [ 'Invoice Rate', t\first(), $this->fetch->invoiceRate(), t\fields(), t\pickKeys('Name', 'Notes') ],
            ),
        );
    }

    public function from() {
        return pipeline(
            $this->fetch->me(),
            t\fields(),
            t\mapAndPickKeys(
                [ 'Name' ],
                [ 'Email' ],
                [ 'Address', fn($address) => explode("\n", $address) ]
            )
        );
    }

    public function client() {
        return pipeline(
            $this->fetch->client(),
            t\fields(),
            t\pickKeys('ContactName', 'Company', 'Website', 'ContactEmail')
        );
    }

}
