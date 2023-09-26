use crate::components::context::LeafletMapContext;
use leptos::*;

#[component(transparent)]
pub fn ImageOverlay(
    #[prop(into)] url: String,
    #[prop(into)] bounds: leaflet::LatLngBounds,
    #[prop(into, optional)] opacity: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] alt: Option<MaybeSignal<String>>,
    #[prop(into, optional)] interactive: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] cross_origin: Option<MaybeSignal<String>>,
    #[prop(into, optional)] cross_origin_toggle: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] error_overlay_url: Option<MaybeSignal<String>>,
    #[prop(into, optional)] z_index: Option<MaybeSignal<f64>>,
    #[prop(into, optional)] class_name: Option<MaybeSignal<String>>,
    #[prop(into, optional)] bubbling_mouse_events: Option<MaybeSignal<bool>>,
    #[prop(into, optional)] pane: Option<MaybeSignal<String>>,
    #[prop(into, optional)] attribution: Option<MaybeSignal<String>>,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("map context not found");
    create_effect(move |_| {
        if let Some(map) = map_context.map() {
            logging::log!("Adding image layer: {}", url);
            let mut options = leaflet::ImageOverlayOptions::new();
            if let Some(opacity) = opacity {
                options.opacity(opacity.get_untracked());
            }
            if let Some(alt) = &alt {
                options.alt(&alt.get_untracked());
            }
            if let Some(interactive) = interactive {
                options.interactive(interactive.get_untracked());
            }
            if let Some(cross_origin) = &cross_origin {
                options.cross_origin(&cross_origin.get_untracked());
            }
            if let Some(cross_origin_toggle) = cross_origin_toggle {
                options.cross_origin_toggle(cross_origin_toggle.get_untracked());
            }
            if let Some(error_overlay_url) = &error_overlay_url {
                options.error_overlay_url(&error_overlay_url.get_untracked());
            }
            if let Some(z_index) = z_index {
                options.z_index(z_index.get_untracked());
            }
            if let Some(class_name) = &class_name {
                options.class_name(&class_name.get_untracked());
            }
            if let Some(bubbling_mouse_events) = bubbling_mouse_events {
                options.bubbling_mouse_events(bubbling_mouse_events.get_untracked());
            }
            if let Some(pane) = &pane {
                options.pane(&pane.get_untracked());
            }
            if let Some(attribution) = &attribution {
                options.attribution(&attribution.get_untracked());
            }

            let map_layer = leaflet::ImageOverlay::new_with_options(&url, &bounds, &options);
            map_layer.addTo(&map);
            on_cleanup(move || {
                map_layer.remove();
            });
        }
    });
}
