use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum RuleCommand {}

#[derive(StructOpt, Debug)]
pub enum RuleEnforcementCommand {}

#[derive(StructOpt, Debug)]
pub enum WebhookCommand {}

#[derive(StructOpt, Debug)]
pub enum TimerCommand {}

#[derive(StructOpt, Debug)]
pub enum TraceCommand {}

#[derive(StructOpt, Debug)]
pub enum SensorCommand {}

#[derive(StructOpt, Debug)]
pub enum TriggerCommand {}

#[derive(StructOpt, Debug)]
pub enum TriggerInstanceCommand {}