class LocalAgentToolchain < Formula
  desc "High-performance workstation toolchain for developers and AI coding agents"
  homepage "https://github.com/criggil/LocalAgentToolchain"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/criggil/LocalAgentToolchain/releases/download/v0.1.0/local-agent-toolchain-aarch64-apple-darwin.tar.gz"
      sha256 "d96c5a4a7eaa18a8488404e215f5e78a5b0fad27c1949906d0a8ae071b0a821a"
    else
      url "https://github.com/criggil/LocalAgentToolchain/releases/download/v0.1.0/local-agent-toolchain-x86_64-apple-darwin.tar.gz"
      sha256 "d96c5a4a7eaa18a8488404e215f5e78a5b0fad27c1949906d0a8ae071b0a821a"
    end
  end

  on_linux do
    if Hardware::CPU.intel?
      url "https://github.com/criggil/LocalAgentToolchain/releases/download/v0.1.0/local-agent-toolchain-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "9321cb30b59594eb3649bfc3e88c6d2e1e849401f5f3d2d33b48fcd7123eef95"
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
