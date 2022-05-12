use anchor_lang::prelude::*;

declare_id!("7dLw1UyCZRd7EZs73MWFy7xpfFPm7TMNQzSWLfcpUszY");

#[program]
pub mod blog {
    use super::*;

    pub fn  create_blog(ctx: Context<CreateBlog>) -> Result<()> {
        let blog_account : &mut Account<Blog> = &mut ctx.accounts.blog_account;
        let post_account : &mut Account<Post> = &mut ctx.accounts.post_account;
        let authority : &Signer  = &mut ctx.accounts.authority;
  
        // sets the blog state
        blog_account.authority = authority.key();
        blog_account.current_post_key = post_account.key();
        Ok(())
    }

    pub fn signup_user (ctx: Context<SignupUser>, name:String , avatar: String) -> Result <()> {

        let user_account : &mut Account<User> = &mut ctx.accounts.user_account;
        let authority : &Signer = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.avatar = avatar;
        user_account.authority = authority.key();

        Ok(())
    }

    pub fn create_post (ctx : Context<CreatePost>, title:String,content : String) -> Result <()>  {

        let blog_account : &mut Account<Blog> = &mut ctx.accounts.blog_account;
        let post_account : &mut Account<Post> = &mut ctx.accounts.post_account;
        let user_account : &mut Account<User> = &mut ctx.accounts.user_account;
        let authority  : &Signer = &mut ctx.accounts.authority;
        let clock: Clock = Clock::get().unwrap();


        post_account.title = title;
        post_account.content = content;
        post_account.timestamp = clock.unix_timestamp;
        post_account.user = user_account.key();
        post_account.authority = authority.key();
        post_account.pre_post_key = blog_account.current_post_key;

        // store created post id as current post id in blog account
        blog_account.current_post_key = post_account.key();

        emit!(PostEvent {
            label: "CREATE".to_string(),
            post_id: post_account.key(),
            next_post_id: None // same as null
        });

        Ok(())

    }

    pub fn update_user(ctx: Context<UpdateUser>, name: String, avatar: String) -> Result <()> {
        let user_account : &mut Account<User>= &mut ctx.accounts.user_account;
  
        user_account.name = name;
        user_account.avatar = avatar;
  
        Ok(())
    }

    pub fn update_post(ctx: Context<UpdatePost>, title: String, content: String) -> Result <()> {
        let post_account : &mut Account<Post> = &mut ctx.accounts.post_account;

        post_account.title = title;
        post_account.content = content;

        emit!(PostEvent {
            label: "UPDATE".to_string(),
            post_id: post_account.key(),
            next_post_id: None // null
        });

        Ok(())
    }

    pub fn delete_post(ctx: Context<DeletePost>) -> Result <()> {
        let post_account : &mut Account<Post> = &mut ctx.accounts.post_account;
        let next_post_account : &mut Account<Post> = &mut ctx.accounts.next_post_account;

        next_post_account.pre_post_key = post_account.pre_post_key;

        emit!(PostEvent {
            label: "DELETE".to_string(),
            post_id: post_account.key(),
            next_post_id: Some(next_post_account.key())
        });

        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreateBlog <'info> {

    #[account(init, payer = authority, space = 8 + 32 + 32)]
    pub blog_account: Account<'info, Blog>,
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 32 + 8)]
    pub post_account: Account<'info, Post>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Blog {
    pub current_post_key: Pubkey,
    pub authority : Pubkey,
}

#[derive(Accounts)]
pub struct SignupUser<'info> {
    #[account(init, payer = authority, space = 8 + 40 + 120  + 32)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub authority : Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
  pub struct UpdateUser<'info> {
      #[account(mut,has_one = authority)]
      pub user_account: Account<'info, User>,
      #[account(mut)]
      pub authority: Signer<'info>,
  }

#[account]
pub struct User {
    pub name: String,
    pub avatar: String,
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = authority, space = 8 + 50 + 500 + 32 + 32 + 32)]
    pub post_account: Account<'info, Post>,
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub blog_account: Account<'info, Blog>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
  pub struct UpdatePost<'info> {
      #[account(mut,has_one = authority)]
      pub post_account: Account<'info, Post>,
      #[account(mut)]
      pub authority: Signer<'info>,
  }

  #[derive(Accounts)]
  pub struct DeletePost<'info> {
      #[account( mut, has_one = authority, close = authority, constraint = post_account.key() == next_post_account.pre_post_key )]
      pub post_account: Account<'info, Post>,
      #[account(mut)]
      pub next_post_account: Account<'info, Post>,
      #[account(mut)]
      pub authority: Signer<'info>,
  }

#[account]
pub struct Post {
   pub title: String,
    pub content: String,
    pub user: Pubkey,
    pub timestamp: i64,
    pub pre_post_key: Pubkey,
    pub authority: Pubkey,
}

#[event]
pub struct PostEvent {
    pub label: String, // label is like 'CREATE', 'UPDATE', 'DELETE'
    pub post_id: Pubkey, // created post
    pub next_post_id: Option<Pubkey>, // for now ignore this, we will use this when we emit delete event
}