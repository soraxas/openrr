// This file is @generated by openrr-internal-codegen.
// It is not intended for manual editing.

#![allow(unused_variables)]
#![allow(clippy::useless_conversion, clippy::unit_arg)]

use abi_stable::{
    rtry,
    std_types::{RBox, RDuration, ROk, RResult, RStr},
};

use super::*;
pub(crate) type PluginTraitObject = RPluginTrait_TO<RBox<()>>;
#[sabi_trait]
pub(crate) trait RPluginTrait: Send + Sync + 'static {
    fn new_gamepad(&self, args: RString) -> RResult<ROption<crate::GamepadProxy>, RError>;
    fn new_joint_trajectory_client(
        &self,
        args: RString,
    ) -> RResult<ROption<crate::JointTrajectoryClientProxy>, RError>;
    fn new_localization(&self, args: RString)
        -> RResult<ROption<crate::LocalizationProxy>, RError>;
    fn new_move_base(&self, args: RString) -> RResult<ROption<crate::MoveBaseProxy>, RError>;
    fn new_navigation(&self, args: RString) -> RResult<ROption<crate::NavigationProxy>, RError>;
    fn new_speaker(&self, args: RString) -> RResult<ROption<crate::SpeakerProxy>, RError>;
    fn new_transform_resolver(
        &self,
        args: RString,
    ) -> RResult<ROption<crate::TransformResolverProxy>, RError>;
}
impl<T> RPluginTrait for T
where
    T: crate::Plugin,
{
    fn new_gamepad(&self, args: RString) -> RResult<ROption<crate::GamepadProxy>, RError> {
        ROk(rtry!(crate::Plugin::new_gamepad(self, args.into()))
            .map(crate::GamepadProxy::new)
            .into())
    }

    fn new_joint_trajectory_client(
        &self,
        args: RString,
    ) -> RResult<ROption<crate::JointTrajectoryClientProxy>, RError> {
        ROk(rtry!(crate::Plugin::new_joint_trajectory_client(
            self,
            args.into()
        ))
        .map(crate::JointTrajectoryClientProxy::new)
        .into())
    }

    fn new_localization(
        &self,
        args: RString,
    ) -> RResult<ROption<crate::LocalizationProxy>, RError> {
        ROk(rtry!(crate::Plugin::new_localization(self, args.into()))
            .map(crate::LocalizationProxy::new)
            .into())
    }

    fn new_move_base(&self, args: RString) -> RResult<ROption<crate::MoveBaseProxy>, RError> {
        ROk(rtry!(crate::Plugin::new_move_base(self, args.into()))
            .map(crate::MoveBaseProxy::new)
            .into())
    }

    fn new_navigation(&self, args: RString) -> RResult<ROption<crate::NavigationProxy>, RError> {
        ROk(rtry!(crate::Plugin::new_navigation(self, args.into()))
            .map(crate::NavigationProxy::new)
            .into())
    }

    fn new_speaker(&self, args: RString) -> RResult<ROption<crate::SpeakerProxy>, RError> {
        ROk(rtry!(crate::Plugin::new_speaker(self, args.into()))
            .map(crate::SpeakerProxy::new)
            .into())
    }

    fn new_transform_resolver(
        &self,
        args: RString,
    ) -> RResult<ROption<crate::TransformResolverProxy>, RError> {
        ROk(
            rtry!(crate::Plugin::new_transform_resolver(self, args.into()))
                .map(crate::TransformResolverProxy::new)
                .into(),
        )
    }
}
pub(crate) type LocalizationTraitObject = RLocalizationTrait_TO<RBox<()>>;
#[abi_stable::sabi_trait]
pub(crate) trait RLocalizationTrait: Send + Sync + 'static {
    fn current_pose(&self, frame_id: RStr<'_>) -> RResult<RIsometry2F64, RError>;
}
impl<T> RLocalizationTrait for T
where
    T: arci::Localization + 'static,
{
    fn current_pose(&self, frame_id: RStr<'_>) -> RResult<RIsometry2F64, RError> {
        ROk(rtry!(arci::Localization::current_pose(self, frame_id.into())).into())
    }
}
pub(crate) type MoveBaseTraitObject = RMoveBaseTrait_TO<RBox<()>>;
#[abi_stable::sabi_trait]
pub(crate) trait RMoveBaseTrait: Send + Sync + 'static {
    fn send_velocity(&self, velocity: RBaseVelocity) -> RResult<(), RError>;
    fn current_velocity(&self) -> RResult<RBaseVelocity, RError>;
}
impl<T> RMoveBaseTrait for T
where
    T: arci::MoveBase + 'static,
{
    fn send_velocity(&self, velocity: RBaseVelocity) -> RResult<(), RError> {
        ROk(rtry!(arci::MoveBase::send_velocity(self, &velocity.into())).into())
    }

    fn current_velocity(&self) -> RResult<RBaseVelocity, RError> {
        ROk(rtry!(arci::MoveBase::current_velocity(self)).into())
    }
}
pub(crate) type NavigationTraitObject = RNavigationTrait_TO<RBox<()>>;
#[abi_stable::sabi_trait]
pub(crate) trait RNavigationTrait: Send + Sync + 'static {
    fn send_goal_pose(
        &self,
        goal: RIsometry2F64,
        frame_id: RStr<'_>,
        timeout: RDuration,
    ) -> RResult<RBlockingWait, RError>;
    fn cancel(&self) -> RResult<(), RError>;
}
impl<T> RNavigationTrait for T
where
    T: arci::Navigation + 'static,
{
    fn send_goal_pose(
        &self,
        goal: RIsometry2F64,
        frame_id: RStr<'_>,
        timeout: RDuration,
    ) -> RResult<RBlockingWait, RError> {
        ROk(rtry!(arci::Navigation::send_goal_pose(
            self,
            goal.into(),
            frame_id.into(),
            timeout.into()
        ))
        .into())
    }

    fn cancel(&self) -> RResult<(), RError> {
        ROk(rtry!(arci::Navigation::cancel(self)).into())
    }
}
pub(crate) type SpeakerTraitObject = RSpeakerTrait_TO<RBox<()>>;
#[abi_stable::sabi_trait]
pub(crate) trait RSpeakerTrait: Send + Sync + 'static {
    fn speak(&self, message: RStr<'_>) -> RResult<RBlockingWait, RError>;
}
impl<T> RSpeakerTrait for T
where
    T: arci::Speaker + 'static,
{
    fn speak(&self, message: RStr<'_>) -> RResult<RBlockingWait, RError> {
        ROk(rtry!(arci::Speaker::speak(self, message.into())).into())
    }
}
pub(crate) type TransformResolverTraitObject = RTransformResolverTrait_TO<RBox<()>>;
#[abi_stable::sabi_trait]
pub(crate) trait RTransformResolverTrait: Send + Sync + 'static {
    fn resolve_transformation(
        &self,
        from: RStr<'_>,
        to: RStr<'_>,
        time: RSystemTime,
    ) -> RResult<RIsometry3F64, RError>;
}
impl<T> RTransformResolverTrait for T
where
    T: arci::TransformResolver + 'static,
{
    fn resolve_transformation(
        &self,
        from: RStr<'_>,
        to: RStr<'_>,
        time: RSystemTime,
    ) -> RResult<RIsometry3F64, RError> {
        ROk(rtry!(arci::TransformResolver::resolve_transformation(
            self,
            from.into(),
            to.into(),
            rtry!(time.try_into())
        ))
        .into())
    }
}
