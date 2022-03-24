require 'kramdown'
require 'fileutils'

Dir.chdir(ARGV.shift)

FileUtils.mkdir_p('build')

Dir.glob('*.md') do |path|
    html_path = path.gsub(/\.md$/, '.html')
    if html_path == 'README.html'
        html_path = 'index.html'
    end

    File.open(File.join('build', html_path), 'w') do |f|
        f.write(<<END)
<meta charset="utf-8"/>
<style>
body {
    font-family: sans-serif;
}
.inner {
    margin: 1rem;
    max-width: 50rem;
    line-height: 1.5;
}
</style>
<body><div class="inner">
END
        f.write(Kramdown::Document.new(File.read(path)).to_html)
    end
end
