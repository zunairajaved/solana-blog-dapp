import * as  anchor from "@project-serum/anchor";
// import { Program } from "@project-serum/anchor";
// import { Blog } from "../target/types/blog";
const assert = require("assert");
const { SystemProgram } = anchor.web3;
describe("blog", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Blog 
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  it("initialize blog account", async () => {
    // call the utility function
    const blog = anchor.web3.Keypair.generate(); // creates random keypair
    const post = anchor.web3.Keypair.generate(); // creates random keypair
    await program.rpc.createBlog({
      accounts: {
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        blogAccount : blog.publicKey,
        postAccount : post.publicKey,
      },
      signers: [blog, post],
    });
    const blogAccount = await program.account.blog.fetch(blog.publicKey);
  
    assert.equal(
      blogAccount.currentPostKey.toString(),
      post.publicKey.toString()
    );
    assert.equal(
      blogAccount.authority.toString(),
      provider.wallet.publicKey.toString()
    );
  });


  it("signup a new user", async () => {
    const user = anchor.web3.Keypair.generate();

    await program.rpc.signupUser("zunaira", "https://img.link", {
      accounts: {
        authority: provider.wallet.publicKey,
        userAccount: user.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [user],
    });
  
    const userAccount = await program.account.user.fetch(user.publicKey);

    assert.equal(userAccount.name, "zunaira");
    assert.equal(userAccount.avatar, "https://img.link");

    assert.equal(
      userAccount.authority.toString(),
      provider.wallet.publicKey.toString()
    );
  });

  it("creates a new post", async () => {

    // blog account
    const blog = anchor.web3.Keypair.generate(); // creates random keypair
    const post = anchor.web3.Keypair.generate(); // creates random keypair
    const user = anchor.web3.Keypair.generate();

    await program.rpc.createBlog({
      accounts: {
        authority: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
        blogAccount : blog.publicKey,
        postAccount : post.publicKey,
      },
      signers: [blog, post],
    });
    const blogAccount = await program.account.blog.fetch(blog.publicKey);

    console.log(blogAccount);
   // user account
   await program.rpc.signupUser("zunaira", "https://img.link", {
     accounts: {
       authority: provider.wallet.publicKey,
       userAccount: user.publicKey,
       systemProgram: SystemProgram.programId,
     },
     signers: [user],
   });
 
   const userAccount = await program.account.user.fetch(user.publicKey);
console.log(userAccount);

   // post account 

   const title = "post title";
   const content = "post content";
 
   await program.rpc.createPost(title, content, {
     // pass arguments to the program
     accounts: {
       blogAccount: blog.publicKey,
       authority: provider.wallet.publicKey,
       userAccount: user.publicKey,
       postAccount: post.publicKey,
       systemProgram: SystemProgram.programId,
     },
     signers: [post],
   });
 
   const postAccount = await program.account.post.fetch(post.publicKey);

   console.log(postAccount);

    assert.equal(postAccount.title, title);
    assert.equal(postAccount.content, content);
    assert.equal(postAccount.user.toString(), user.publicKey.toString());
    assert.equal(postAccount.prePostKey.toString(), blogAccount.currentPostKey.toString());
    assert.equal(
      postAccount.authority.toString(),
      provider.wallet.publicKey.toString()
    );
    assert.ok(postAccount.timestamp);
  });
});
