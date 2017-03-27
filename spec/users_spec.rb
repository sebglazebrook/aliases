require "json"

describe "`users` command" do

  let(:docker_command) { DockerCommand.new(command, args, dockerfile) }
  let(:args) { [] }
  let(:dockerfile) { DockerfileRepository.find(:initialized) }
  subject { docker_command.invoke }

  after { docker_command.kill }

  context "without any subcommand args" do

    let(:command ) { "bash -c 'cd /tmp && /code/target/debug/aliases users'" }

    context "without any custom users" do

      it "lists the default user" do
        expect(subject.output).to match("default")
      end
    end

    context "given there are custom users" do

      before do
        docker_command.start_container
        docker_command.query("bash -c 'cd ~ && /code/target/debug/aliases init --user superman'")
      end

      it "lists all the users" do
        expect(subject.output).to match("default.*superman")
      end
    end
  end

  context "with subcommand `move`" do

    context "when given a username that exists" do

      context "when given a priority value higher than it's current" do

        it "moves the user up to the value given"
      end
      context "when given a priority value lower than it's current" do

        it "moves the user down to the value given"
      end
      context "when given a priority value well beyond the lowest user" do

        it "moves the user to the lowest priority"
      end

      context "when given a negative priority value" do

        it "moves the users priority down by the given number"
      end
    end

    context "when the given username does NOT exist" do

      it "has a error exit code"
      it "returns a sensible error message"
    end
  end

  context "with subcommand `use`" do
    context "when the user's priority is NOT number 1" do

      it "moves the user to the top of the priority list"
    end

    context "when the user's priority is number 1" do

      it "leaves the user at the top of the priority list"
    end
  end

  context "with subcommand `enable`" do

    let(:command ) { "bash -c 'cat ~/.aliases_cfg'" } # TODO use an alias command instead

    context "when the user is disabled" do

      before do
        docker_command.start_container
        docker_command.query("bash -c 'cd ~ && /code/target/debug/aliases init --user superman'")
        docker_command.query("bash -c 'cd ~ && /code/target/debug/aliases disable superman'")
      end

      it "becomes enabled" do
        expect(JSON.parse(subject.output)["disabled_users"]).to eq nil # TODO this should default to an empty array not nil
      end
    end

    context "when the user is already enabled" do

      before do
        docker_command.start_container
        docker_command.query("bash -c 'cd ~ && /code/target/debug/aliases init --user superman'")
      end

      it "stays enabled" do
        expect(JSON.parse(subject.output)["disabled_users"]).to eq nil # TODO this should default to an empty array not nil
      end
    end
  end

  context "with subcommand `disable`" do

    let(:command) { "bash -c 'cd /tmp && /code/target/debug/aliases users disable superman'" }

    context "when the user is enabled" do

      before do
        docker_command.start_container
        docker_command.query("bash -c 'cd ~ && /code/target/debug/aliases init --user superman'")
      end

      it "makes them disabled" do
        subject
        expect(JSON.parse(docker_command.query("bash -c 'cat ~/.aliases_cfg'"))["disabled_users"].include?("superman")).to eq true
      end
    end

    context "when the user is disabled" do

      before do
        docker_command.start_container
        docker_command.query("bash -c 'cd ~ && /code/target/debug/aliases init --user superman'")
        docker_command.query("bash -c 'cd ~ && /code/target/debug/aliases users disable superman'")
      end

      it "leaves them disabled" do
        subject
        expect(JSON.parse(docker_command.query("bash -c 'cat ~/.aliases_cfg'"))["disabled_users"].include?("superman")).to eq true
      end
    end

    it "means their aliases no longer work"
  end
end
