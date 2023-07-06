class med < Formula
    desc "A simple enough CLI tool with auditable capability for Data Masking/Encyption/Decryption for CSV/JSON files."
    homepage "https://github.com/jayhuang75/rust-cli-med"
    url "https://github.com/jayhuang75/rust-cli-med/releases/download/0.6.2/macos_x86_archive-0.6.2.tar.gz"
    sha256 "b35ee2ee615549f3b27741d8f1055ae94deb40b10601b678c6185542e48f0d01"
    license "Apache-2.0"
    version "0.6.2"
  
    def install
      bin.install "med"
    end
  end