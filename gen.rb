require 'kramdown'
require 'fileutils'

FileUtils.mkdir_p('build')

Dir.glob('*.md') do |path|
    html_path = path.gsub(/\.md$/, '.html')
    if html_path == 'README.html'
        html_path = 'index.html'
    end

    File.open(File.join('build', html_path), 'w') do |f|
        f.write(Kramdown::Document.new(File.read(path)).to_html)
    end
end
