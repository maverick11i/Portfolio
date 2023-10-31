use yew::{function_component, html, Html};

/// A Yew functional component representing the user's profile section.
#[function_component]
pub fn Profile() -> Html {
    html! {
        <>
            <div class="container px-5 my-5 fadein">
                <div class="text-center mb-5">
                    <h1 class="display-5 fw-bolder mb-0"><span class="text-gradient d-inline">{"About"}</span></h1>
                </div>
                <div class="row gx-5 justify-content-center">
                    <div class="col-lg-11 col-xl-9 col-xxl-8">
                        <section>
                            <div class="d-flex align-items-center justify-content-between mb-4">
                                <h2 class="text-primary fw-bolder mb-0">{"Experience"}</h2>
                            </div>
                            <div class="card shadow border-0 rounded-4 mb-5">
                                <div class="card-body p-5">
                                    <div class="row align-items-center gx-5">
                                        <div class="col text-center text-lg-start mb-4 mb-lg-0">
                                            <div class="bg-light p-4 rounded-4">
                                                <div class="text-primary fw-bolder mb-2">{"2023 October - now"}</div>
                                                <div class="small fw-bolder">{"Software Engineer"}</div>
                                                <div class="small text-muted">{"会社 R"}</div>
                                                <div class="small text-muted">{"Tokyo"}</div>
                                            </div>
                                        </div>
                                        <div class="col-lg-8"><div>{"SES事業で様々な業務に携わる"}</div></div>
                                    </div>
                                </div>
                            </div>
                            <div class="card shadow border-0 rounded-4 mb-5">
                                <div class="card-body p-5">
                                    <div class="row align-items-center gx-5">
                                        <div class="col text-center text-lg-start mb-4 mb-lg-0">
                                            <div class="bg-light p-4 rounded-4">
                                                <div class="text-primary fw-bolder mb-2">{"2023 - 2023 August"}</div>
                                                <div class="small fw-bolder">{"Software Engineer"}</div>
                                                <div class="small text-muted">{"会社 F"}</div>
                                                <div class="small text-muted">{"Tokyo"}</div>
                                            </div>
                                        </div>
                                        <div class="col-lg-8"><div>{"社内製品の開発に携わる"}</div></div>
                                    </div>
                                </div>
                            </div>
                        </section>
                        <section>
                            <h2 class="text-secondary fw-bolder mb-4">{"Education"}</h2>
                            <div class="card shadow border-0 rounded-4 mb-5">
                                <div class="card-body p-5">
                                    <div class="row align-items-center gx-5">
                                        <div class="col text-center text-lg-start mb-4 mb-lg-0">
                                            <div class="bg-light p-4 rounded-4">
                                                <div class="text-secondary fw-bolder mb-2">{"2019 - 2022"}</div>
                                                <div class="mb-2">
                                                    <div class="small fw-bolder">{"HAL Tokyo"}</div>
                                                    <div class="small text-muted">{"Tokyo"}</div>
                                                </div>
                                                <div class="fst-italic">
                                                    <div class="small text-muted">{"Undergraduate"}</div>
                                                    <div class="small text-muted">{"Computer Science"}</div>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="col-lg-8">
                                            <div>{"高度情報処理学科 サイバーセキュリティ専攻"}<br />{"コンピューターサイエンス、基礎的なプログラミング、サーバー構築など幅広く学ぶ"}</div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>
                        <div class="pb-5"></div>
                        <section>
                            <div class="card shadow border-0 rounded-4 mb-5">
                                <div class="card-body p-5">
                                    <div class="mb-5">
                                        <div class="d-flex align-items-center mb-4">
                                            <div class="feature bg-primary bg-gradient-primary-to-secondary text-white rounded-3 me-3"><i class="bi bi-tools"></i></div>
                                            <h3 class="fw-bolder mb-0"><span class="text-gradient d-inline">{"Middleware"}</span></h3>
                                        </div>
                                        <div class="row row-cols-1 row-cols-md-3 mb-4">
                                            <div class="col mb-4 mb-md-0"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"Nginx"}</div></div>
                                            <div class="col mb-4 mb-md-0"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"Apach Tomcat"}</div></div>
                                            <div class="col"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"Docker"}</div></div>

                                        </div>
                                        <div class="row row-cols-1 row-cols-md-3">
                                            <div class="col mb-4 mb-md-0"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"PostgreSQL"}</div></div>
                                            <div class="col mb-4 mb-md-0"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"MySQL"}</div></div>
                                            <div class="col"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"MSSQL"}</div></div>
                                        </div>
                                    </div>
                                    <div class="mb-0">
                                        <div class="d-flex align-items-center mb-4">
                                            <div class="feature bg-primary bg-gradient-primary-to-secondary text-white rounded-3 me-3"><i class="bi bi-code-slash"></i></div>
                                            <h3 class="fw-bolder mb-0"><span class="text-gradient d-inline">{"Language"}</span></h3>
                                        </div>
                                        <div class="row row-cols-1 row-cols-md-3 mb-4">
                                            <div class="col mb-4 mb-md-0"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"Rust"}</div></div>
                                            <div class="col mb-4 mb-md-0"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"C#"}</div></div>
                                            <div class="col"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"Java"}</div></div>
                                        </div>
                                        <div class="row row-cols-1 row-cols-md-3">
                                            <div class="col mb-4 mb-md-0"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"JavaScript"}</div></div>
                                            <div class="col mb-4 mb-md-0"><div class="d-flex align-items-center bg-light rounded-4 p-3 h-100">{"PHP"}</div></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>
                    </div>
                </div>
            </div>
        </>
    }
}
