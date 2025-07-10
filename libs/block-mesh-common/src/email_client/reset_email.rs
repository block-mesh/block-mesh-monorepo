pub const RESET_EMAIL: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title></title>

    <!--[if !mso]><!-->
    <style type="text/css">
        @import url('https://fonts.mailersend.com/css?family=Inter:400,600');
    </style>

     <style>
        body {
            background-color: #121212;
            color: #ffffff;
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 0;
            -webkit-text-size-adjust: 100%;
            -ms-text-size-adjust: 100%;
        }
        .container {
            width: 100%;
            max-width: 600px;
            margin: 0 auto;
            padding: 20px;
            border: 1px solid #444;
            border-radius: 8px;
            background-color: #1e1e1e;
            box-sizing: border-box;
        }
        .header {
            color: white;
            text-align: center;
            padding-bottom: 20px;
            border-bottom: 1px solid #444;
        }
        .content {
            color: white;
            padding: 20px 0;
        }
        .button {
            display: inline-block;
            padding: 10px 20px;
            color: #ffffff;
            background-color: #ff5722;
            border-radius: 4px;
            text-decoration: none;
            font-weight: bold;
        }
        .footer {
            text-align: center;
            padding-top: 20px;
            border-top: 1px solid #444;
            font-size: 12px;
            color: #888;
        }
        @media only screen and (max-width: 600px) {
            .container {
                padding: 10px;
            }
            .header img {
                width: 100px;
                height: 100px;
            }
            .button {
                padding: 10px;
                font-size: 16px;
            }
        }
    </style>

    <!--<![endif]-->

    <style type="text/css" rel="stylesheet" media="all">
        @media only screen and (max-width: 640px) {
            .ms-header {
                display: none !important;
            }
            .ms-content {
                width: 100% !important;
                border-radius: 0;
            }
            .ms-content-body {
                padding: 30px !important;
            }
            .ms-footer {
                width: 100% !important;
            }
            .mobile-wide {
                width: 100% !important;
            }
            .info-lg {
                padding: 30px;
            }
        }
    </style>
    <!--[if mso]>
    <style type="text/css">
    body { font-family: Arial, Helvetica, sans-serif!important  !important; }
    td { font-family: Arial, Helvetica, sans-serif!important  !important; }
    td * { font-family: Arial, Helvetica, sans-serif!important  !important; }
    td p { font-family: Arial, Helvetica, sans-serif!important  !important; }
    td a { font-family: Arial, Helvetica, sans-serif!important  !important; }
    td span { font-family: Arial, Helvetica, sans-serif!important  !important; }
    td div { font-family: Arial, Helvetica, sans-serif!important  !important; }
    td ul li { font-family: Arial, Helvetica, sans-serif!important  !important; }
    td ol li { font-family: Arial, Helvetica, sans-serif!important  !important; }
    td blockquote { font-family: Arial, Helvetica, sans-serif!important  !important; }
    th * { font-family: Arial, Helvetica, sans-serif!important  !important; }
    </style>
    <![endif]-->
</head>
<body>
    <div class="container">
        <div class="header">
            <img src="https://perceptron-network.perceptrons.xyz/Logo_only_white.png" alt="BlockMesh Network" width="128" height="128" />
<h1>Perceptron Network - Reset Password</h1>
</div>
<div class="content">
<p style="color:white">Hi,</p>
<p style="color:white">You have requested to reset your password.</p>
<p style="color:white">Please click the following link to continue:</p>
<a href="{{action_url}}" class="button">Click Here</a>
</div>
<div class="footer">
<p>&copy; Perceptron Network</p>
</div>
</div>
</body>
</html>
"#;
