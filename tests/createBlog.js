const anchor = require("@project-serum/anchor");

const { SystemProgram } = anchor.web3;

// we will discus the parameters when we use it
async function createBlog(program, provider) {
  const blog = anchor.web3.Keypair.generate(); // creates random keypair
  const post = anchor.web3.Keypair.generate(); // creates random keypair

  await program.rpc.createBlog({
    accounts: {
      authority: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
      blog : blog.publicKey,
      post : post.publicKey,
    },
    signers: [blog, post],
  });
  const blogAccount = await program.account.blog.fetch(blog.publicKey);


  return { blogAccount, blog, post };
}

module.exports = {
  createBlog,
};