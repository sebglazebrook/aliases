class Logger

  def self.info(message)
    puts message if verbose?
  end

  private

  def self.verbose?
    ENV["VERBOSE"] == "true"
  end

end
