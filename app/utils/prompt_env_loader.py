"""
Utility to load all .env files from prompt_envs directory
and make them available as environment variables
"""
import os
import glob
import re


def load_prompt_envs():
    """
    Load all .env files from prompt_envs directory into environment variables.
    This makes all prompts available via os.environ.
    """
    # Get the project root directory
    current_dir = os.path.dirname(__file__)
    project_root = os.path.abspath(os.path.join(current_dir, '..', '..'))
    prompt_envs_dir = os.path.join(project_root, 'prompt_envs')
    
    if not os.path.exists(prompt_envs_dir):
        print(f"Warning: prompt_envs directory not found at {prompt_envs_dir}")
        return {}
    
    # Find all .env files in prompt_envs directory
    env_files = glob.glob(os.path.join(prompt_envs_dir, '.env.*'))
    
    loaded_prompts = {}
    
    for env_file in env_files:
        try:
            # Read the .env file manually
            with open(env_file, 'r', encoding='utf-8') as f:
                content = f.read().strip()
                
            # More robust parsing for KEY = "value" format with multiline content
            # Find the first = sign to separate key and value
            if '=' not in content:
                print(f"⚠️ No assignment found in {os.path.basename(env_file)}")
                continue
                
            first_eq = content.find('=')
            key_part = content[:first_eq].strip()
            value_part = content[first_eq + 1:].strip()
            
            # Remove surrounding quotes if present
            if ((value_part.startswith('"') and value_part.endswith('"')) or 
                (value_part.startswith("'") and value_part.endswith("'"))):
                value_part = value_part[1:-1]
                
            if key_part and value_part:
                os.environ[key_part] = value_part
                loaded_prompts[key_part] = len(value_part)
                print(f"✅ Loaded prompt '{key_part}' ({len(value_part)} chars) from {os.path.basename(env_file)}")
            else:
                print(f"⚠️ Invalid format in {os.path.basename(env_file)}: empty key or value")
            
        except Exception as e:
            print(f"❌ Error loading {env_file}: {e}")
    
    print(f"🎯 Total prompts loaded as environment variables: {len(loaded_prompts)}")
    return loaded_prompts


def get_prompt(prompt_name: str) -> str:
    """
    Get a prompt from environment variables.
    
    Args:
        prompt_name: Name of the prompt environment variable
        
    Returns:
        The prompt content as string, or empty string if not found
    """
    return os.environ.get(prompt_name, "")


def list_available_prompts() -> dict:
    """
    List all available prompt environment variables.
    
    Returns:
        Dictionary with prompt names and their content lengths
    """
    prompt_vars = {}
    
    for key, value in os.environ.items():
        if key.startswith('prompt_'):
            prompt_vars[key] = len(value) if value else 0
    
    return prompt_vars


# Auto-load prompts when module is imported
if __name__ != "__main__":
    load_prompt_envs()
