require 'kramdown'
require 'fileutils'
require 'erb'

class Page
    def initialize(md_path)
        @md_path = md_path
        @doc = Kramdown::Document.new(File.read(@md_path))
        @title = @doc.root.children[0].children[0].value
    end
    attr_reader :title

    def html
        @doc.to_html
    end

    def html_path
        path = @md_path.gsub(/\.md$/, '.html')
        if path == 'README.html'
            'index.html'
        else
            path
        end
    end
end

def generate_file_list(pages, html_path)
    html = pages.filter do |page|
        page.html_path != html_path
    end.map do |page|
        "<li><a href=#{page.html_path}>#{page.title}</a></li>"
    end.join("\n")
    
    "<ul>#{html}</ul>"
end

t = ERB.new(File.open('template.html.erb').read)

Dir.chdir(ARGV.shift)

FileUtils.mkdir_p('build')

pages = Dir.glob('*.md').map do |md|
    Page.new(md)
end

pages.each do |page|
    File.open(File.join('build', page.html_path), 'w') do |f|
        title = page.title
        body = page.html
        footer = ''
        if page.html_path == 'index.html'
            footer = generate_file_list(pages, page.html_path)
        end
        f.write(t.result(binding))
    end
end
