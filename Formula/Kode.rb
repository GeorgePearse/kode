class Kode < Formula
  desc "Terminal coding agent"
  homepage "https://github.com/just-every/kode"
  version "v0.2.67"
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/just-every/kode/releases/download/v0.2.67/kode-aarch64-apple-darwin.tar.gz"
      sha256 "ed8fbb68d8cff0f76d28c9c9b69445dab66f8e645613e9145061e950e8cf7507"
    else
      url "https://github.com/just-every/kode/releases/download/v0.2.67/kode-x86_64-apple-darwin.tar.gz"
      sha256 "642f656b1d45fe305738f519b5d44c8329bd9963f3e3781ee542a4313d2102f7"
    end
  end

  def install
    bin.install Dir["kode-*"].first => "kode"
    # Provide a compatibility shim
    (bin/"koder").write <<~EOS
      #!/bin/bash
      exec "#{bin}/kode" "$@"
    EOS
  end

  test do
    system "#{bin}/kode", "--help"
  end
end
