use crate::host_capabilities::verification::{KeylessInfo, KeylessPrefixInfo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod crypto;
pub mod net;
pub mod oci;
pub mod verification;

/// SigstoreVerificationInputV1 is used for the v1/verify callback
#[derive(Serialize, Deserialize, Debug)]
pub enum SigstoreVerificationInputV1 {
    /// Require the verification of the manifest digest of an OCI object (be
    /// it an image or anything else that can be stored into an OCI registry)
    /// to be signed by Sigstore, using public keys mode
    SigstorePubKeyVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of PEM encoded keys that must have been used to sign the OCI object
        pub_keys: Vec<String>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    /// Require the verification of the manifest digest of an OCI object to be
    /// signed by Sigstore, using keyless mode
    SigstoreKeylessVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of keyless signatures that must be found
        keyless: Vec<KeylessInfo>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },
}

/// SigstoreVerificationInputV2 is used for the v2/verify callback
/// From now on we use serde internally tagged.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SigstoreVerificationInputV2 {
    /// Require the verification of the manifest digest of an OCI object (be
    /// it an image or anything else that can be stored into an OCI registry)
    /// to be signed by Sigstore, using public keys mode
    SigstorePubKeyVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of PEM encoded keys that must have been used to sign the OCI object
        pub_keys: Vec<String>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    /// Require the verification of the manifest digest of an OCI object to be
    /// signed by Sigstore, using keyless mode
    SigstoreKeylessVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of keyless signatures that must be found
        keyless: Vec<KeylessInfo>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    /// Require the verification of the manifest digest of an OCI object to be
    /// signed by Sigstore using keyless mode, where the passed subject is a URL
    /// prefix of the subject to match
    SigstoreKeylessPrefixVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// List of keyless signatures that must be found
        keyless_prefix: Vec<KeylessPrefixInfo>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    /// Require the verification of the manifest digest of an OCI object to be
    /// signed by Sigstore using keyless mode and performed in GitHub Actions
    SigstoreGithubActionsVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// owner of the repository. E.g: octocat
        owner: String,
        /// Optional - Repo of the GH Action workflow that signed the artifact. E.g: example-repo
        repo: Option<String>,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },

    /// Require the verification of the manifest digest of an OCI object
    /// using the user provided certificate
    SigstoreCertificateVerify {
        /// String pointing to the object (e.g.: `registry.testing.lan/busybox:1.0.0`)
        image: String,
        /// PEM encoded certificate used to verify the signature
        certificate: Vec<u8>,
        /// Optional - the certificate chain that is used to verify the provided
        /// certificate. When not specified, the certificate is assumed to be trusted
        certificate_chain: Option<Vec<Vec<u8>>>,
        /// Require the  signature layer to have a Rekor bundle.
        /// Having a Rekor bundle allows further checks to be performed,
        /// like ensuring the signature has been produced during the validity
        /// time frame of the certificate.
        ///
        /// It is recommended to set this value to `true` to have a more secure
        /// verification process.
        require_rekor_bundle: bool,
        /// Optional - Annotations that must have been provided by all signers when they signed the OCI artifact
        annotations: Option<HashMap<String, String>>,
    },
}

pub mod crypto_v1 {
    use crate::host_capabilities::crypto::Certificate;
    use serde::{Deserialize, Serialize};

    /// CertificateVerificationRequest holds information about a certificate and
    /// a chain to validate it with.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct CertificateVerificationRequest {
        /// PEM-encoded certificate
        pub cert: Certificate,
        /// list of PEM-encoded certs, ordered by trust usage (intermediates first, root last)
        pub cert_chain: Option<Vec<Certificate>>,
        /// RFC 3339 time format string, to check expiration against. If None,
        /// certificate is assumed never expired
        pub not_after: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct CertificateVerificationResponse {
        pub trusted: bool,
    }
}
