class Uncomment < Formula
  desc "A CLI tool to uncomment code"
  homepage "https://github.com/Kishan-Agarwal-28/uncomment"
  url "https://github.com/Kishan-Agarwal-28/uncomment/releases/download/v0.1.0/uncomment-macos-amd64"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"
  version "0.1.0"

  def install
    bin.install "uncomment-macos-amd64" => "uncomment"
  end

  test do
    system "#{bin}/uncomment", "--version"
  end
end
