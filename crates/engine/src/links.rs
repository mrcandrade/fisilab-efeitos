//! Community and project links (shown in Help ▸, the About dialog, the start screen and the
//! Tools bar).
//!
//! FisiEfeitos is the Suíte FisiLab (UFRGS) version of EffectCraft: the links point at the suite,
//! its manifesto, the upstream project (credited, and the home of the documentation) and this
//! version's own issue tracker. The suite URLs are the same as `fisilab_brand::SUITE_URL` and
//! `fisilab_brand::MANIFESTO_URL` (the engine stays below the egui layer, so it can't depend on
//! that crate; a ui-egui test keeps them equal).

pub const APP_NAME: &str = "FisiEfeitos";
pub const APP_SLUG: &str = "fisiefeitos";
/// The project FisiEfeitos is based on.
pub const UPSTREAM_NAME: &str = "EffectCraft";
/// Site da Suíte FisiLab.
pub const WEBSITE: &str = "https://github.com/mrcandrade/larfic/tree/main/suite-fisilab";
/// Manifesto pelas tecnologias livres.
pub const MANIFESTO: &str = "https://github.com/mrcandrade/larfic/blob/main/suite-fisilab/MANIFESTO.md";
/// FisiEfeitos's own page (source code, releases).
pub const APP_PAGE: &str = "https://github.com/mrcandrade/fisiefeitos";
/// The upstream project (EffectCraft, by the ArtCraft Team and contributors), whose
/// documentation Help ▸ opens.
pub const GITHUB: &str = "https://github.com/storytold/effectcraft";
/// Reportar um problema.
pub const ISSUES: &str = "https://github.com/mrcandrade/fisiefeitos/issues";
