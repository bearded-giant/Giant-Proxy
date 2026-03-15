class GiantProxy < Formula
  desc "HTTPS proxy with Map Remote rules"
  homepage "https://github.com/recharge/giant-proxy"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/recharge/giant-proxy/releases/download/v#{version}/giant-proxy-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
    on_intel do
      url "https://github.com/recharge/giant-proxy/releases/download/v#{version}/giant-proxy-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/recharge/giant-proxy/releases/download/v#{version}/giant-proxy-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER"
    end
    on_intel do
      url "https://github.com/recharge/giant-proxy/releases/download/v#{version}/giant-proxy-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  def install
    bin.install "giantd"
    bin.install "giant-proxy"
  end

  def post_install
    ohai "Run `giant-proxy init` to set up CA certificate"
  end

  test do
    assert_match "giant-proxy", shell_output("#{bin}/giant-proxy version")
  end
end
