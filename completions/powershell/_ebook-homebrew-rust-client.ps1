
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'ebook-homebrew-rust-client' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'ebook-homebrew-rust-client'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'ebook-homebrew-rust-client' {
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'CLI version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'CLI version')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'check server status')
            [CompletionResult]::new('upload', 'upload', [CompletionResultType]::ParameterValue, 'upload image files')
            [CompletionResult]::new('convert', 'convert', [CompletionResultType]::ParameterValue, 'convert image files to PDF')
            [CompletionResult]::new('download', 'download', [CompletionResultType]::ParameterValue, 'download converted PDF file')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'ebook-homebrew-rust-client;status' {
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'ebook-homebrew host URL')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'ebook-homebrew port')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'ebook-homebrew-rust-client;upload' {
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'ebook-homebrew host URL')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'ebook-homebrew port')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'ebook-homebrew-rust-client;convert' {
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'ebook-homebrew host URL')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'ebook-homebrew port')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'ebook-homebrew-rust-client;download' {
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'ebook-homebrew host URL')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'ebook-homebrew port')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'ebook-homebrew-rust-client;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
