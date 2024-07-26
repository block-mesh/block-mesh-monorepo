use crate::frontends::new_frontend_webserver::components::{
    Heading, IfLetSome, Subheading, Table, TableCell, TableHead, TableHeader,
};
use leptos::*;
use serde::{Deserialize, Serialize};
use tailwind_fuse::*;

#[component]
pub fn RefererRank(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] step: u8,
    #[prop(into, optional)] is_complete: bool,
) -> impl IntoView {
    let class = tw_merge!(
        "absolute left-0 top-0 w-1 lg:bottom-0 lg:top-auto lg:w-full",
        is_complete.then_some("bg-blue shadow-light h-px")
    );

    view! {
        <li class="relative lg:flex-1 refer-card">
            <div class="rounded-t-md border border-b-0 border-gray-200 lg:border-0">
                <div class="refer-card-wrapper">
                    <span class=class aria-hidden="true"></span>
                    <span class="flex items-start px-6 py-5 text-sm font-medium">
                        <span class="flex-shrink-0">
                            <Show
                                when=move || is_complete
                                fallback=move || {
                                    view! {
                                        <span class="flex h-10 w-10 items-center justify-center rounded-full border-2 border-darkOrange">
                                            <span class="text-gray-500 text-darkOrange">
                                                {step.to_string()}
                                            </span>
                                        </span>
                                    }
                                }
                            >

                                <span class="flex h-10 w-10 items-center justify-center rounded-full bg-blue">
                                    <svg
                                        class="h-6 w-6 text-dark"
                                        viewBox="0 0 24 24"
                                        fill="currentColor"
                                        aria-hidden="true"
                                    >
                                        <path
                                            fillRule="evenodd"
                                            d="M19.916 4.626a.75.75 0 01.208 1.04l-9 13.5a.75.75 0 01-1.154.114l-6-6a.75.75 0 011.06-1.06l5.353 5.353 8.493-12.739a.75.75 0 011.04-.208z"
                                            clipRule="evenodd"
                                        ></path>
                                    </svg>
                                </span>
                            </Show>

                        </span>

                        <span class="ml-4 mt-0.5 flex min-w-0 flex-col">
                            <span class="text-sm font-medium text-darkOrange">{title}</span>
                            <span class="text-sm font-medium text-gray-500">{description}</span>
                        </span>
                    </span>
                </div>
                <Show when=move || step != 1>
                    <div class="absolute inset-0 top-0 hidden w-3 lg:block" aria-hidden="true">
                        <svg
                            class="h-full w-full text-gray-300"
                            viewBox="0 0 12 82"
                            fill="none"
                            preserveAspectRatio="none"
                        >
                            <path
                                d="M0.5 0V31L10.5 41L0.5 51V82"
                                stroke="currentcolor"
                                vectorEffect="non-scaling-stroke"
                            ></path>
                        </svg>
                    </div>
                </Show>
            </div>
        </li>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Referral {
    id: i64,
    name: String,
    email: String,
    start_date: String,
    reward: u8,
}

async fn get_referrals(_: ()) -> Vec<Referral> {
    vec![
        Referral {
            id: 1,
            name: "Sean Mckinney".to_string(),
            email: "sean.mckney@gmail.com".to_string(),
            start_date: "May 9, 2024".to_string(),
            reward: 10,
        },
        Referral {
            id: 2,
            name: "Laura Smith".to_string(),
            email: "laura.smith@example.com".to_string(),
            start_date: "June 10, 2024".to_string(),
            reward: 20,
        },
        Referral {
            id: 3,
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            start_date: "July 15, 2024".to_string(),
            reward: 15,
        },
        Referral {
            id: 4,
            name: "Alice Johnson".to_string(),
            email: "alice.johnson@example.com".to_string(),
            start_date: "August 1, 2024".to_string(),
            reward: 25,
        },
        Referral {
            id: 5,
            name: "Michael Brown".to_string(),
            email: "michael.brown@example.com".to_string(),
            start_date: "September 5, 2024".to_string(),
            reward: 30,
        },
        Referral {
            id: 6,
            name: "Emily Davis".to_string(),
            email: "emily.davis@example.com".to_string(),
            start_date: "October 12, 2024".to_string(),
            reward: 18,
        },
        Referral {
            id: 7,
            name: "Daniel Wilson".to_string(),
            email: "daniel.wilson@example.com".to_string(),
            start_date: "November 20, 2024".to_string(),
            reward: 22,
        },
        Referral {
            id: 8,
            name: "Sophia Martinez".to_string(),
            email: "sophia.martinez@example.com".to_string(),
            start_date: "December 8, 2024".to_string(),
            reward: 28,
        },
        Referral {
            id: 9,
            name: "James Lee".to_string(),
            email: "james.lee@example.com".to_string(),
            start_date: "January 10, 2025".to_string(),
            reward: 35,
        },
        Referral {
            id: 10,
            name: "Olivia Harris".to_string(),
            email: "olivia.harris@example.com".to_string(),
            start_date: "February 14, 2025".to_string(),
            reward: 40,
        },
        Referral {
            id: 11,
            name: "Christopher White".to_string(),
            email: "christopher.white@example.com".to_string(),
            start_date: "March 18, 2025".to_string(),
            reward: 12,
        },
        Referral {
            id: 12,
            name: "Jessica Lewis".to_string(),
            email: "jessica.lewis@example.com".to_string(),
            start_date: "April 25, 2025".to_string(),
            reward: 27,
        },
    ]
}

#[component]
pub fn Orders() -> impl IntoView {
    let referral_resource = Resource::new(|| {}, get_referrals);

    view! {
        <div class="flex items-end justify-between gap-4">
            <Heading>Referrals</Heading>
            <button class="-my-0.5 cursor-pointer">
                <span class="material-symbols-outlined">link</span>
                Copy Referer Link
            </button>
        </div>

        <div class="referer-ranking my-12">
            <div>
                <Subheading class="mt-14">Ranking</Subheading>
                <nav class="mt-4 mx-auto max-w-7xl" aria-label="Progress">
                    <ol role="list" class="rounded-md lg:flex lg:rounded-none ">
                        <RefererRank
                            title="Iron"
                            description="Entry-level tasks completed"
                            step=1
                            is_complete=true
                        />
                        <RefererRank
                            title="Bronze"
                            description="Basic proficiency achieved"
                            step=2
                            is_complete=true
                        />
                        <RefererRank
                            title="Silver"
                            description="Intermediate skills demonstrated"
                            step=3
                            is_complete=false
                        />
                        <RefererRank
                            title="Gold"
                            description="Advanced capabilities shown"
                            step=4
                            is_complete=false
                        />
                        <RefererRank
                            title="Diamond"
                            description="Expert level mastery attained"
                            step=5
                            is_complete=false
                        />
                    </ol>
                </nav>
            </div>

        </div>

        <Subheading class="mt-14">Referrals List</Subheading>
        <Table class="mt-4 [--gutter:theme(spacing.6)] lg:[--gutter:theme(spacing.10)]">
            <TableHead>
                <tr>
                    <TableHeader>Name</TableHeader>
                    <TableHeader>Email</TableHeader>
                    <TableHeader>Start Date</TableHeader>
                    <TableHeader class="text-right">My Reward</TableHeader>
                </tr>
            </TableHead>
            <tbody>
                <Suspense>
                    <IfLetSome opt=Signal::derive(move || referral_resource.get()) let:referrals>
                        {referrals
                            .clone()
                            .into_iter()
                            .map(|referral| {
                                view! {
                                    <tr>
                                        <TableCell>{referral.name.clone()}</TableCell>
                                        <TableCell>{referral.email.clone()}</TableCell>
                                        <TableCell>{referral.start_date.clone()}</TableCell>
                                        <TableCell class="text-right">
                                            {referral.reward.to_string()}
                                        </TableCell>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </IfLetSome>
                </Suspense>

            </tbody>
        </Table>
    }
}
