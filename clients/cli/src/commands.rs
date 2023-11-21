pub mod action;
pub mod automation;
pub mod datastore;
pub mod iam;
pub mod setup;

use action::{
    ActionAliasCommand, ActionCommand, ExecutionCommand, InquiryCommand, RunCommand,
    WorkflowCommand,
};
use automation::{
    RuleCommand, RuleEnforcementCommand, SensorCommand, TimerCommand, TraceCommand, TriggerCommand,
    TriggerInstanceCommand, WebhookCommand,
};
use datastore::KeyCommand;
use iam::{
    ApiKeyCommand, AuthCommand, LoginCommand, RoleAssignmentCommand, RoleCommand, WhoAmICommand,
};
use setup::{PackCommand, PolicyCommand, PolicyTypeCommand, RunnerCommand, ServiceRegistryCommand};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "synapse", about = "CLI for Synapse.")]
pub struct CLI {
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]

pub enum Command {
    #[structopt(about = "Invoke an action manually.")]
    Run(RunCommand),
    #[structopt(about = "An activity that happens as a response to the external event.")]
    Action(ActionCommand),
    #[structopt(name = "action-alias", about = "Action aliases.")]
    ActionAlias(ActionAliasCommand),
    #[structopt(about = "Authenticate user and acquire access token.")]
    Auth(AuthCommand),
    #[structopt(
        about = "Authenticate user, acquire access token, and update CLI config directory"
    )]
    Login(LoginCommand),
    #[structopt(name = "whoami", about = "Display the currently authenticated user")]
    WhoAmI(WhoAmICommand),
    #[structopt(about = "API Keys.")]
    ApiKey(ApiKeyCommand),
    #[structopt(about = "An invocation of an action.")]
    Execution(ExecutionCommand),
    #[structopt(
        about = "Inquiries provide an opportunity to ask a question and wait for a response in a workflow."
    )]
    Inquiry(InquiryCommand),
    #[structopt(
        about = "Key value pair is used to store commonly used configuration for reuse in sensors, actions, and rules."
    )]
    Key(KeyCommand),
    #[structopt(about = "A group of related integration resources: actions, rules, and sensors.")]
    Pack(PackCommand),
    #[structopt(about = "Policy that is enforced on a resource.")]
    Policy(PolicyCommand),
    #[structopt(
        name = "policy-type",
        about = "Type of policy that can be applied to resources."
    )]
    PolicyType(PolicyTypeCommand),
    #[structopt(
        about = "A specification to invoke an 'action' on a 'trigger' selectively based on some criteria."
    )]
    Rule(RuleCommand),
    #[structopt(about = "Webhooks.")]
    Webhook(WebhookCommand),
    #[structopt(about = "Timers.")]
    Timer(TimerCommand),
    #[structopt(about = "Runner is a type of handler for a specific class of actions.")]
    Runner(RunnerCommand),
    #[structopt(
        about = "An adapter which allows you to integrate StackStorm with external system."
    )]
    Sensor(SensorCommand),
    #[structopt(about = "A group of executions, rules and triggerinstances that are related.")]
    Trace(TraceCommand),
    #[structopt(
        about = "An external event that is mapped to a st2 input. It is the st2 invocation point."
    )]
    Trigger(TriggerCommand),
    #[structopt(
        name = "trigger-instance",
        about = "Actual instances of triggers received by st2."
    )]
    TriggerInstance(TriggerInstanceCommand),
    #[structopt(
        name = "rule-enforcement",
        about = "Models that represent enforcement of rules."
    )]
    RuleEnforcement(RuleEnforcementCommand),
    #[structopt(
        about = "Commands for workflow authoring related operations. Only orquesta workflows are supported."
    )]
    Workflow(WorkflowCommand),
    #[structopt(
        name = "service-registry",
        about = "Service registry group and membership related commands."
    )]
    ServiceRegistry(ServiceRegistryCommand),
    #[structopt(about = "RBAC roles.")]
    Role(RoleCommand),
    #[structopt(name = "role-assignment", about = "RBAC role assignments.")]
    RoleAssignment(RoleAssignmentCommand),
    // ... repeat for other data structures like executions, enforcements, etc.
}
