describe "`exec` command" do

  let(:docker_command) { DockerCommand.new(command, args, dockerfile) }
  subject { docker_command.invoke }

  after { docker_command.kill }
  let(:args) { [] }

  let(:dockerfile) { DockerfileRepository.find(:initialized) }

  let(:command) { "bash -c 'cd #{directory} && aliases exec #{directory} #{alias_name}'" }

  context "when given a directory" do

    let(:directory) { "/tmp/target-dir" }

    context "and an alias name" do

      let(:alias_name) { "awesome" }

      context "and an alias exists" do

        before do
          docker_command.start_container
          docker_command.query("bash -c 'cd /tmp && mkdir -p target-dir && cd target-dir && aliases add awesome pwd'")
        end

        describe "default behavior" do

          it "prints out the command that will be executed" do
            expect(subject.output).to match(/Executing: pwd/)
          end

          it "executes the command" do
            expect(subject.output).to match(/\/tmp\/target-dir/)
          end
        end

        context "when there are forwarding arguments" do

          let(:alias_name) { "great" }

          let(:command) { "bash -c 'cd #{directory} && aliases exec #{directory} #{alias_name} -- -la'" }

          before do
            docker_command.start_container
            docker_command.query("bash -c 'cd /tmp && mkdir -p target-dir && cd target-dir && aliases add #{alias_name} ls'")
          end

          it "prints out the command and the arguments that will be executed" do
            expect(subject.output).to match(/Executing: ls -la/)
          end

          it "executes the command" do
            expect(subject.output).to match(/\.aliases/)
          end
        end
      end

      context "and an alias does NOT exist" do

        it "does something"
      end
    end
  end

  context "when given a symlinked directory" do
    let(:directory) { "/tmp/linked" }

    context "and an alias name" do

      let(:alias_name) { "awesome" }

      context "and an alias exists" do

        before do
          docker_command.start_container
          docker_command.query("bash -c 'cd /tmp && mkdir -p original && ln -s original linked && cd linked && aliases add awesome pwd'")
        end

        describe "default behavior" do

          it "prints out the command that will be executed" do
            expect(subject.output).to match(/Executing: pwd/)
          end

          #it "executes the command" do
            #expect(subject.output).to match(/\/tmp\/linked/)
          #end
        end
      end
    end
  end
end
