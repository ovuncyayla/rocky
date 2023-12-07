
#[derive(Debug)]
pub struct RouteConfig {
    pub routes: Vec<RouteDef>
}

#[derive(Debug)]
pub struct RouteDef {
    pub path: String,
    pub conf: jg::Config
}

