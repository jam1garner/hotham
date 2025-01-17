use openxr::{Duration, HapticVibration};

use crate::resources::{HapticContext, XrContext};

pub fn apply_haptic_feedback(xr_context: &mut XrContext, haptic_context: &mut HapticContext) {
    if haptic_context.amplitude_this_frame == 0. {
        return;
    }

    let duration = Duration::from_nanos(1e+7 as _);
    let frequency = 180.;

    let event = HapticVibration::new()
        .amplitude(haptic_context.amplitude_this_frame)
        .frequency(frequency)
        .duration(duration);

    xr_context
        .haptic_feedback_action
        .apply_feedback(
            &xr_context.session,
            xr_context.right_hand_subaction_path,
            &event,
        )
        .expect("Unable to apply haptic feedback!");

    // Reset the value
    haptic_context.amplitude_this_frame = 0.;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simple smoke test.
    #[test]
    pub fn apply_haptic_feedback_test() {
        let (mut xr_context, _) = XrContext::new().unwrap();
        let mut haptic_context = HapticContext::default();

        apply_haptic_feedback(&mut xr_context, &mut haptic_context);
    }
}
