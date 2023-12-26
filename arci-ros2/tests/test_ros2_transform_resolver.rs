mod shared;

use std::time::Duration;

use arci::TransformResolver;
use arci_ros2::{
    msg::{
        geometry_msgs::{Quaternion, TransformStamped, Vector3},
        tf2_msgs::TFMessage,
    },
    utils::convert_ros2_time_to_system_time,
};
use assert_approx_eq::assert_approx_eq;
use ros2_client::builtin_interfaces::Time;
use shared::*;

const RETRY_RATE: f64 = 50.0;
const MAX_RETRY: usize = 100;

const FRAME_ID_1: &str = "map";
const FRAME_ID_2: &str = "odom";
const FRAME_ID_3: &str = "base_link";
const TF_TRA_X: f64 = 1.2;
const TF_TRA_Y: f64 = 2.3;
const TF_TRA_Z: f64 = 3.4;

// Roll: 0, Pitch: 0, Yaw: 1.5707963(=PI/2)
const TF_ROT_W: f64 = std::f64::consts::FRAC_1_SQRT_2;
const TF_ROT_X: f64 = 0.;
const TF_ROT_Y: f64 = 0.;
const TF_ROT_Z: f64 = std::f64::consts::FRAC_1_SQRT_2;

// Transform from "base_link" to "map"
const EXPECTED_TRA_X: f64 = -1.1;
const EXPECTED_TRA_Y: f64 = 3.5;
const EXPECTED_TRA_Z: f64 = -6.8;
const EXPECTED_ROT_W: f64 = 0.;
const EXPECTED_ROT_X: f64 = 0.;
const EXPECTED_ROT_Y: f64 = 0.;
const EXPECTED_ROT_Z: f64 = -1.;

#[tokio::test(flavor = "multi_thread")]
async fn test_transform_resolver() {
    let node = test_node();
    let tf_topic = node.create_topic::<TFMessage>("/tf").unwrap();
    let tf_publisher = node.create_publisher::<TFMessage>(&tf_topic).unwrap();

    let ros2_transform_resolver = arci_ros2::Ros2TransformResolver::new(
        node.clone(),
        Duration::from_millis(100),
        RETRY_RATE,
        MAX_RETRY,
    );

    let dummy_ros2_time_first = Time::now();
    std::thread::sleep(Duration::from_millis(10));
    let dummy_ros2_time_middle = Time::now();
    std::thread::sleep(Duration::from_millis(10));
    let dummy_ros2_time_last = Time::now();

    let dummy_time_middle = convert_ros2_time_to_system_time(&dummy_ros2_time_middle);

    let mut tf_stamped_common = TransformStamped::default();
    tf_stamped_common.header.stamp = dummy_ros2_time_first;
    tf_stamped_common.transform.translation = Vector3 {
        x: TF_TRA_X,
        y: TF_TRA_Y,
        z: TF_TRA_Z,
    };
    tf_stamped_common.transform.rotation = Quaternion {
        x: TF_ROT_X,
        y: TF_ROT_Y,
        z: TF_ROT_Z,
        w: TF_ROT_W,
    };

    let mut tf_odom_stamped_first = tf_stamped_common.clone();
    tf_odom_stamped_first.header.frame_id = FRAME_ID_1.to_string();
    tf_odom_stamped_first.child_frame_id = FRAME_ID_2.to_string();

    let mut tf_base_stamped_first = tf_stamped_common;
    tf_base_stamped_first.header.frame_id = FRAME_ID_2.to_string();
    tf_base_stamped_first.child_frame_id = FRAME_ID_3.to_string();

    let mut tf_odom_stamped_last = tf_odom_stamped_first.clone();
    tf_odom_stamped_last.header.stamp = dummy_ros2_time_last;

    let mut tf_base_stamped_last = tf_base_stamped_first.clone();
    tf_base_stamped_last.header.stamp = dummy_ros2_time_last;

    let tf = vec![
        tf_odom_stamped_first,
        tf_base_stamped_first,
        tf_odom_stamped_last,
        tf_base_stamped_last,
    ];
    let tf_message = TFMessage { transforms: tf };

    tokio::spawn(async move {
        for _ in 0..2 {
            tf_publisher.publish(tf_message.clone()).unwrap();
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    tokio::time::sleep(Duration::from_secs(5)).await;

    let tf_received = ros2_transform_resolver
        .resolve_transformation(FRAME_ID_3, FRAME_ID_1, dummy_time_middle)
        .unwrap();

    assert_approx_eq!(tf_received.translation.x, EXPECTED_TRA_X);
    assert_approx_eq!(tf_received.translation.y, EXPECTED_TRA_Y);
    assert_approx_eq!(tf_received.translation.z, EXPECTED_TRA_Z);
    assert_approx_eq!(tf_received.rotation.w, EXPECTED_ROT_W);
    assert_approx_eq!(tf_received.rotation.i, EXPECTED_ROT_X);
    assert_approx_eq!(tf_received.rotation.j, EXPECTED_ROT_Y);
    assert_approx_eq!(tf_received.rotation.k, EXPECTED_ROT_Z);
}
