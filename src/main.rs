use argh::FromArgs;
use http::Request;
use hyper::service::Service;

use hyper::Body;
use third_wheel::*;

/// Run a TLS mitm proxy that does no modification to the traffic
#[derive(FromArgs)]
struct StartMitm {
    /// port to bind proxy to
    #[argh(option, short = 'p', default = "8080")]
    port: u16,

    /// pem file for self-signed certificate authority certificate
    #[argh(option, short = 'c', default = "\"ca/ca_certs/cert.pem\".to_string()")]
    cert_file: String,

    /// pem file for private signing key for the certificate authority
    #[argh(option, short = 'k', default = "\"ca/ca_certs/key.pem\".to_string()")]
    key_file: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: StartMitm = argh::from_env();
    let ca = CertificateAuthority::load_from_pem_files_with_passphrase_on_key(
        &args.cert_file,
        &args.key_file,
        "third-wheel",
    )?;
    let trivial_mitm =
        mitm_layer(|req: Request<Body>, mut third_wheel: ThirdWheel| third_wheel.call(req));
    let mitm_proxy = MitmProxy::builder(trivial_mitm, ca).build();
    let (_, mitm_proxy_fut) = mitm_proxy.bind(format!("127.0.0.1:{}", args.port).parse().unwrap());
    mitm_proxy_fut.await.unwrap();
    Ok(())
}
