class DockerCommand

  CONTAINER_NAME = "aliases-test-container"
  IMAGE_NAME = "aliases-test-image"

  attr_reader :output

  def initialize(command, args, dockerfile)
    @command, @args, @dockerfile = command, args, dockerfile
  end

  def invoke
    start_container
    Logger.info "---- Running command: #{@command} #{@args}"
    Logger.info "docker exec -ti #{CONTAINER_NAME} #{@command} #{@args.join(" ")}"
    @output = `docker exec -ti #{CONTAINER_NAME} #{@command} #{@args.join(" ")}`
    self
  end

  def query(command)
    Logger.info "---- Running command: #{command}"
    Logger.info "docker exec -ti #{CONTAINER_NAME} #{command}"
    `docker exec -ti #{CONTAINER_NAME} #{command}`
  end

  def diff
    `docker diff #{CONTAINER_NAME}`.split("\n")
  end

  def kill
    Logger.info '---- Killing container'
    run_command("docker rm --force #{CONTAINER_NAME}")
  end

  def start_container
    build_container
    Logger.info '---- Starting container'
    run_command("docker run -ti -v ${APP_ROOT}:/code -d --workdir /code --name #{CONTAINER_NAME} #{IMAGE_NAME} sh")
  end

  private

  def build_container
    if ENV["REBUILD_CONTAINER"] || !system("docker images -q #{IMAGE_NAME}:latest | grep #{IMAGE_NAME}")
      Logger.info '---- Building container'
      run_command("docker build --tag #{IMAGE_NAME} --file #{@dockerfile} .")
    end
  end


  def run_command(command_string)
    if verbose?
      Logger.info command_string
      system(command_string)
    else
      `#{command_string}`
    end
  end

  def verbose?
    ENV["VERBOSE"] == "true"
  end

end
