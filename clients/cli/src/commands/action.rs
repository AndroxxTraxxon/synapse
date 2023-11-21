use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt, Debug)]
pub enum ActionCommand{
    #[structopt(about = "Get the list of actions.")]
    List,
    #[structopt(about = "Get an individual action.")]
    Get{
        #[structopt(short, long, help = "Reference Name or ID of the action.")]
        ref_or_id: String
    },
    #[structopt(about = "Create a new action.")]
    Create{
        #[structopt(parse(from_os_str), help="JSON/YAML file containing the action to create.")]
        file: PathBuf,
    },
    #[structopt(about = "Update an existing action.")]
    Update{
        #[structopt(short, long, help = "Reference Name or ID of the action.")]
        ref_or_id: String,
        #[structopt(parse(from_os_str), help="JSON/YAML file containing the action to create.")]
        file: PathBuf,
    },
    #[structopt(about = "Delete an existing action.")]
    Delete{
        #[structopt(short, long, help = "Reference Name or ID of the action")]
        ref_or_id: String
    },
    #[structopt(about = "Clone a new action from an existing action.")]
    Clone{
        #[structopt(short, long, help = "Reference Name or ID of the action")]
        source_ref_or_id: String,
        #[structopt(short = "p", long, help = "The name of the pack which will contain the cloned action")]
        dest_pack_name: String,
        #[structopt(short = "n", long, help = "The new name of the cloned action")]
        dest_action_name: String,
    },
    #[structopt(about = "Enable an existing action.")]
    Enable{
        #[structopt(short, long, help = "Reference Name or ID of the action")]
        ref_or_id: String
    },
    #[structopt(about = "Disable an existing action.")]
    Disable{
        #[structopt(short, long, help = "Reference Name or ID of the action")]
        ref_or_id: String
    },
    #[structopt(about = "Execute an action manually.")]
    Execute{
        #[structopt(short, long, help = "Reference Name or ID of the action")]
        ref_or_id: String
    }
}

#[derive(StructOpt, Debug)]
pub enum ActionAliasCommand{
    
}

#[derive(StructOpt, Debug)]
pub enum RunCommand{
    
}

#[derive(StructOpt, Debug)]
pub enum ExecutionCommand{
    
}

#[derive(StructOpt, Debug)]
pub enum InquiryCommand{
    
}

#[derive(StructOpt, Debug)]
pub enum WorkflowCommand{
    
}