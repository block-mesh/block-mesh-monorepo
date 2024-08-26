use crate::frontends::components::heading::Heading;
use crate::frontends::components::sub_heading::Subheading;
use crate::frontends::components::tables::table::Table;
use crate::frontends::components::tables::table_cell::TableCell;
use crate::frontends::components::tables::table_head::TableHead;
use crate::frontends::components::tables::table_header::TableHeader;
use crate::frontends::context::webapp_context::WebAppContext;
use crate::frontends::new_frontend_webserver::app::application_layout::ApplicationLayout;
use block_mesh_common::interfaces::server_api::LeaderBoardUser;
use leptos::*;

#[component]
pub fn DailyLeaderboardDashboard() -> impl IntoView {
    let async_data = WebAppContext::get_daily_leaderboard();
    let day = create_rw_signal("".to_string());
    let users: Signal<Vec<LeaderBoardUser>> = Signal::derive(move || {
        if let Some(Some(j)) = async_data.get() {
            day.set(j.day.to_string());
            j.leaderboard_users
        } else {
            vec![]
        }
    });

    view! {
        <ApplicationLayout>
            <div class="flex items-start justify-start gap-4">
                <Heading>Top 5 <span class="pr-2 pl-2">-</span> Daily Leaderboard</Heading>
            </div>
            <Subheading class="mt-14">
                Daily Users Ranking <span class="pr-2 pl-2">|</span> {day}
                <span class="pr-2 pl-2">|</span> not including perks or referal bonus
            </Subheading>
            <Table class="mt-4 [--gutter:theme(spacing.6)] lg:[--gutter:theme(spacing.10)]">
                <TableHead>
                    <tr>
                        <TableHeader>Rank</TableHeader>
                        <TableHeader>Email</TableHeader>
                        <TableHeader class="text-right">Points</TableHeader>
                    </tr>
                </TableHead>
                <tbody>
                    <Suspense>
                        {users
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(index, user)| {
                                view! {
                                    <tr>
                                        <TableCell>{index + 1}</TableCell>
                                        <TableCell>{user.email.clone()}</TableCell>
                                        <TableCell class="text-right">
                                            {format!("{:.1}", user.points.unwrap_or_default())}
                                        </TableCell>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </Suspense>

                </tbody>
            </Table>
        </ApplicationLayout>
    }
}
