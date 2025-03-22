use std::marker::PhantomData;

use dioxus::prelude::*;

use crate::use_motion;

use super::utility::TransitionVariant;
#[derive(Clone)]
pub enum AnimatedRouterContext<R: Routable + PartialEq> {
    /// Transition from one route to another.
    FromTo(R, R),
    /// Settled in a route.
    In(R),
}

impl<R: Routable + PartialEq> AnimatedRouterContext<R> {
    /// Get the current destination route.
    pub fn target_route(&self) -> &R {
        match self {
            Self::FromTo(_, to) => to,
            Self::In(to) => to,
        }
    }

    /// Update the destination route.
    pub fn set_target_route(&mut self, to: R) {
        match self {
            Self::FromTo(old_from, old_to) => {
                *old_from = old_to.clone();
                *old_to = to
            }
            Self::In(old_to) => *self = Self::FromTo(old_to.clone(), to),
        }
    }

    /// After the transition animation has finished, make the outlet only render the destination route.
    pub fn settle(&mut self) {
        if let Self::FromTo(_, to) = self {
            *self = Self::In(to.clone())
        }
    }
}

#[component]
/// Renders an outlet that supports animated transitions between routes.
///
/// This function sets up a routing context and monitors changes in the current route to
/// determine when an animated transition should occur. When a transition is detected and
/// the layout depth or route conditions are met, it renders a transition component; otherwise,
/// it renders a standard outlet.
///
/// # Examples
///
/// ```
/// // Assuming `AppRoute` implements `AnimatableRoute`:
/// let animated_outlet = AnimatedOutlet::<AppRoute>();
/// // Use `animated_outlet` as part of your Dioxus component tree.
/// ```
pub fn AnimatedOutlet<R: AnimatableRoute>() -> Element {
    let route = use_route::<R>();
    // Create router context only if we're the root AnimatedOutlet
    let mut prev_route = use_signal(|| AnimatedRouterContext::In(route.clone()));
    use_context_provider(move || prev_route);

    use_effect(move || {
        if prev_route.peek().target_route() != &use_route::<R>() {
            println!(
                "Route changed: {:?} -> {:?}",
                prev_route.peek().target_route().to_string(),
                use_route::<R>().to_string()
            );
            prev_route
                .write()
                .set_target_route(use_route::<R>().clone());
        }
    });

    let outlet: OutletContext<R> = use_outlet_context();

    let from_route: Option<(R, R)> = match prev_route() {
        AnimatedRouterContext::FromTo(from, to) => Some((from, to)),
        _ => None,
    };

    if let Some((from, to)) = from_route {
        // Special handling for transitions from root path
        let is_from_root = from.to_string() == "/";
        let current_depth = outlet.level();
        let target_depth = to.get_layout_depth();
        let from_depth = from.get_layout_depth();

        // Animate if:
        // 1. We're transitioning from root
        // 2. We're at the correct layout depth
        // 3. The target route has a different layout depth than the current route
        if is_from_root || current_depth == target_depth || from_depth != target_depth {
            return rsx! {
                FromRouteToCurrent::<R> {
                    route_type: PhantomData,
                    from: from.clone(),
                    to: to.clone(),
                }
            };
        } else {
            return rsx! {
                Outlet::<R> {}
            };
        }
    } else {
        return rsx! {
            Outlet::<R> {}
        };
    }
}

pub trait AnimatableRoute: Routable + Clone + PartialEq {
    fn get_transition(&self) -> TransitionVariant;
    fn get_component(&self) -> Element;
    fn get_layout_depth(&self) -> usize;
}

/// Shortcut to get access to the [AnimatedRouterContext].
pub fn use_animated_router<Route: Routable + PartialEq>() -> Signal<AnimatedRouterContext<Route>> {
    use_context()
}

#[component]
fn FromRouteToCurrent<R: AnimatableRoute>(route_type: PhantomData<R>, from: R, to: R) -> Element {
    let mut animated_router = use_animated_router::<R>();
    let config = to.get_transition().get_config();
    let from_transform = use_motion(config.exit_start);
    let to_transform = use_motion(config.enter_start);
    let from_opacity = use_motion(1.0f32);
    let to_opacity = use_motion(0.0f32);

    // Track animation state separately
    let mut is_animating = use_signal(|| true);

    // Start animation in a separate effect
    use_effect(move || {
        // Animate FROM route with gentler spring
        from_transform
            .spring()
            .stiffness(80.0) // Reduced from 160.0
            .damping(12.0) // Reduced from 20.0
            .mass(1.0) // Reduced from 1.5
            .on_complete(move || {
                println!("From transform animation complete");
            })
            .animate_to(config.exit_end);

        // Animate TO route with gentler spring
        to_transform
            .spring()
            .stiffness(80.0) // Reduced from 160.0
            .damping(12.0) // Reduced from 20.0
            .mass(1.0) // Reduced from 1.5
            .on_complete(move || {
                println!("To transform animation complete");
            })
            .animate_to(config.enter_end);

        // Fade out old route
        from_opacity
            .spring()
            .stiffness(160.0)
            .damping(20.0)
            .mass(1.5)
            .on_complete(move || {
                println!("From opacity animation complete");
            })
            .animate_to(0.0);

        // Fade in new route
        to_opacity
            .spring()
            .stiffness(160.0)
            .damping(20.0)
            .mass(1.5)
            .on_complete(move || {
                println!("To opacity animation complete");
            })
            .animate_to(1.0);
    });

    // Track animation completion in a separate effect
    use_effect(move || {
        let from_transform_animating = from_transform.is_animating();
        let to_transform_animating = to_transform.is_animating();
        let from_opacity_animating = from_opacity.is_animating();
        let to_opacity_animating = to_opacity.is_animating();

        if !from_transform_animating
            && !to_transform_animating
            && !from_opacity_animating
            && !to_opacity_animating
            && *is_animating.peek()
        {
            is_animating.set(false);
            animated_router.write().settle();
        }
    });

    rsx! {
        div {
            class: "route-container",
            style: "position: relative; overflow-visible;",
            div {
                class: "route-content from",
                style: "
                    transform: translate3d({from_transform.get().x}%, {from_transform.get().y}%, 0) 
                             scale({from_transform.get().scale_x}, {from_transform.get().scale_y});
                    opacity: {from_opacity.get()};
                    will-change: transform, opacity;
                    backface-visibility: hidden;
                    -webkit-backface-visibility: hidden;
                ",
                {from.render(from.get_layout_depth() + 1)}
            }
            div {
                class: "route-content to",
                style: "
                    transform: translate3d({to_transform.get().x}%, {to_transform.get().y}%, 0) 
                             scale({to_transform.get().scale_x}, {to_transform.get().scale_y});
                    opacity: {to_opacity.get()};
                    will-change: transform, opacity;
                    backface-visibility: hidden;
                    -webkit-backface-visibility: hidden;
                ",
                Outlet::<R> {}
            }
        }
    }
}
