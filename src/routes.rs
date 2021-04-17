use crate::handlers;

use actix_web::web;

// Populate the ServiceConfig with all the route needed for the server
pub fn routes(cfg: &mut web::ServiceConfig) {
    // The /health is used only to get a status over the server
    cfg.route("/health", web::get().to(|| async { "zpour" }))
        // Bind the /api/* route
        .service(
            web::scope("/api")
                // TODO - These routes should be secured behind a token verification
                // Or at least just the ingest (POST)
                .route("/hosts", web::post().to(handlers::hosts::host_ingest))
                .route("/hosts", web::get().to(handlers::hosts::host_all))
                .route("/cpu_info", web::get().to(handlers::cpu::cpu_info))
                .route("/loadavg", web::get().to(handlers::cpu::load_avg))
                .route("/disks_info", web::get().to(handlers::disks::disks_info))
                .route("/iostats", web::get().to(handlers::disks::iostats))
                .route("/memory", web::get().to(handlers::memory::memory)),
        );
}
