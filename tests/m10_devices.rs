use std::path::{Path, PathBuf};

use mer3ly_site::devices::{
    AuthorizationState, DeviceCatalog, DeviceStatus, EvidenceState, FlashRecipeState,
    NetworkSupportState, SaleState,
};
use mer3ly_site::pages::devices;
use mer3ly_site::repositories::PublicSiteData;
use mer3ly_site::site::DEVICE_CSS;

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

#[test]
fn catalog_exposes_two_honest_development_specimens() {
    let catalog = DeviceCatalog::load(workspace_root()).expect("validated device catalog");
    let devices = catalog.ordered();

    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0].id, "v4-desktop-radio");
    assert_eq!(devices[1].id, "t114-field-radio");
    for device in devices {
        assert_eq!(device.status, DeviceStatus::DevelopmentSpecimen);
        assert_eq!(device.sale.state, SaleState::NotOffered);
        assert!(device.sale.purchase_url.is_none());
        assert!(device.open_requirement.len() >= 4);
        assert_eq!(device.authorization.state, AuthorizationState::Open);
        assert!(
            device
                .network_support
                .iter()
                .all(|network| network.state == NetworkSupportState::Demonstrated)
        );
        assert_eq!(device.flash_recipe[0].state, FlashRecipeState::Partial);
        assert!(
            device
                .evidence
                .iter()
                .any(|evidence| evidence.state == EvidenceState::Proven)
        );
    }
}

#[test]
fn index_starts_with_roles_and_distinguishes_the_forms() {
    let data = PublicSiteData::load(workspace_root()).expect("public site data");
    let document = devices::index_document_for(&data.devices);

    assert!(document.contains("Start with the job. Keep the recipe open."));
    assert!(document.contains("choose a role"));
    assert!(document.contains("device-silhouette-v4"));
    assert!(document.contains("device-silhouette-t114"));
    assert!(document.contains("href=\"/devices.css?v="));
    assert_eq!(document.matches("data-device-id=").count(), 2);
}

#[test]
fn profile_order_puts_purchase_after_the_open_recipe() {
    let data = PublicSiteData::load(workspace_root()).expect("public site data");
    for device in data.devices.ordered() {
        let document = devices::document_for(device);
        let recipe = document.find("exact recipe state").expect("recipe section");
        let build = document.find("build it").expect("build section");
        let verify = document.find("verify it").expect("verify section");
        let networks = document.find("network support").expect("network section");
        let flash = document.find("install firmware").expect("flash section");
        let authorization = document
            .find("radio authorization")
            .expect("authorization section");
        let purchase = document
            .find("buy assembled hardware")
            .expect("purchase section");

        assert!(
            recipe < build
                && build < verify
                && verify < networks
                && networks < flash
                && flash < authorization
                && authorization < purchase
        );
        assert!(document.contains("data-purchase-status=\"unavailable\""));
        assert!(!document.contains("class=\"button button-primary purchase-link\""));
        assert!(
            document.contains("One selected personality at a time")
                || document.contains("one selected personality at a time")
        );
        assert!(document.contains("TechArticle"));
        assert!(document.contains(&format!("data-device-id=\"{}\"", device.id)));
    }
}

#[test]
fn catalog_layout_has_phone_specific_single_column_ledgers() {
    for contract in [
        ".device-card",
        ".device-silhouette-v4",
        ".device-silhouette-t114",
        ".device-spec-grid",
        ".catalog-choice-grid",
        ".authorization-grid",
        ".purchase-unavailable",
        "@media (max-width: 440px)",
    ] {
        assert!(
            DEVICE_CSS.contains(contract),
            "site CSS is missing {contract}"
        );
    }
}
