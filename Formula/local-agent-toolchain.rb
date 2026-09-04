class LocalAgentToolchain < Formula
  desc "High-performance workstation toolchain for developers and AI coding agents"
  homepage "https://github.com/criggil/LocalAgentToolchain"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/criggil/LocalAgentToolchain/releases/download/v0.1.0/local-agent-toolchain-aarch64-apple-darwin.tar.gz"
      sha256 "54d27238417a7f197c6e4c9bb5effef21b80393cc1c6c7073400d0b11d1f64e5"
    else
      url "https://github.com/criggil/LocalAgentToolchain/releases/download/v0.1.0/local-agent-toolchain-x86_64-apple-darwin.tar.gz"
      sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/criggil/LocalAgentToolchain/releases/download/v0.1.0/local-agent-toolchain-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    end
  end

  def install
    bin.install "task"
    bin.install "note"
    bin.install "skill"
  end

  test do
    system "#{bin}/task", "--help"
    system "#{bin}/note", "--help"
    system "#{bin}/skill", "--help"
  end
end
