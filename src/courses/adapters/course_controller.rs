use crate::{application::course_service::CourseService, domain::course::Course};
use std::sync::Arc;
use warp::Filter;

pub fn course_routes(service: Arc<CourseService<'_>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let all = warp::path("courses").and(warp::get()).and_then({
        let svc = service.clone();
        move || {
            let svc = svc.clone();
            async move {
                let courses: Vec<Course> = svc.get_all().await;
                Ok::<_, warp::Rejection>(warp::reply::json(&courses))
            }
        }
    });

    let by_cycle = warp::path("courses").and(warp::get()).and(warp::query::<std::collections::HashMap<String, String>>()).and_then({
        let svc = service.clone();
        move |params: std::collections::HashMap<String, String>| {
            let svc = svc.clone();
            async move {
                if let Some(cycle_str) = params.get("cycle") {
                    if let Ok(cycle) = cycle_str.parse::<i32>() {
                        let courses = svc.get_by_cycle(cycle).await;
                        return Ok::<_, warp::Rejection>(warp::reply::json(&courses));
                    }
                }
                Ok::<_, warp::Rejection>(warp::reply::json(&Vec::<Course>::new()))
            }
        }
    });

    all.or(by_cycle)
}
